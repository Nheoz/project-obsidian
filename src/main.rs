mod benchmark;
mod cli;
mod embedded;
mod hardware;
mod modules;
mod profiles;
mod reporting;
mod rollback;
mod snapshot;
mod validation;
mod windows;

use anyhow::Result;
use clap::Parser;
use cli::{Cli, Commands};
use colored::*;
use hardware::HardwareInfo;
use modules::{
    ai::AiModule, developer::DeveloperModule, gaming::GamingModule, privacy::PrivacyModule,
};
use profiles::OptimizationProfile;
use rollback::RollbackEngine;
use snapshot::Snapshot;
use std::path::{Path, PathBuf};
use validation::ValidationEngine;
use windows::WindowsInfo;

/// Returns a path resolved relative to the directory containing the executable.
/// Falls back to the provided name if the exe path cannot be determined.
fn exe_dir() -> PathBuf {
    std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|d| d.to_path_buf()))
        .unwrap_or_else(|| PathBuf::from("."))
}

fn main() -> Result<()> {
    #[cfg(windows)]
    let _ = colored::control::set_virtual_terminal(true);

    let args = Cli::parse();

    // If launched with no subcommands (e.g. double-clicked in Explorer by standard user):
    // Relaunch immediately inside a dedicated Administrator Command Prompt console window
    if args.command.is_none() && !args.interactive_terminal {
        if !WindowsInfo::check_is_admin() {
            let current_exe = std::env::current_exe()?;
            let script = format!(
                "Start-Process cmd.exe -ArgumentList '/c \"\"{}\"\" --interactive-terminal' -Verb RunAs",
                current_exe.display()
            );
            let _ = std::process::Command::new("powershell")
                .args(["-NoProfile", "-Command", &script])
                .spawn();
            return Ok(());
        }
    }

    print_banner();

    let windows = WindowsInfo::detect();
    let hardware = HardwareInfo::detect();

    if let Some(cmd) = args.command {
        execute_command(cmd, &windows, &hardware)?;
    } else {
        run_interactive_menu(&windows, &hardware)?;
    }

    Ok(())
}

fn execute_command(cmd: Commands, windows: &WindowsInfo, hardware: &HardwareInfo) -> Result<()> {
    match cmd {
        Commands::Status => {
            print_system_overview(windows, hardware);
            PrivacyModule::audit()?;
            GamingModule::audit()?;
            DeveloperModule::audit()?;
        }

        Commands::Analyze => {
            println!(
                "{}",
                "[*] INITIATING NON-DESTRUCTIVE DRY-RUN ANALYSIS"
                    .cyan()
                    .bold()
            );
            print_system_overview(windows, hardware);
            PrivacyModule::audit()?;
            GamingModule::audit()?;
            DeveloperModule::audit()?;
            AiModule::doctor()?;
            println!(
                "\n{}",
                "[V] Analysis complete. Zero changes applied to disk or registry."
                    .green()
                    .bold()
            );
        }

        Commands::Doctor => {
            AiModule::doctor()?;
            ValidationEngine::run_all()?;
        }

        Commands::Benchmark { label } => {
            let metrics = benchmark::BenchmarkMetrics::capture(&label);
            metrics.print_summary();
            let out_file = format!("benchmark-{}.json", label);
            metrics.save_to_file(Path::new(&out_file))?;
            println!("  [OK] Saved benchmark metrics to: {}", out_file.green());
        }

        Commands::Validate => {
            ValidationEngine::run_all()?;
        }

        Commands::Restore { snapshot } => {
            RollbackEngine::execute(&exe_dir().join("obsidian-state"), snapshot)?;
        }

        Commands::Apply { profile, dry_run } => {
            let opt_profile = OptimizationProfile::from_type(profile);
            println!(
                "{} [{}] (DryRun: {})",
                "[*] APPLYING PROFILE:".cyan().bold(),
                opt_profile.name.yellow().bold(),
                dry_run
            );

            if !windows.is_admin && !dry_run {
                println!(
                    "{}",
                    "[*] Administrator privileges required to configure system."
                        .yellow()
                        .bold()
                );
                println!("{}", "[*] Launching Windows UAC elevation prompt...".cyan());
                WindowsInfo::relaunch_as_admin()?;
                return Ok(());
            }

            let mut snap = Snapshot::new(&opt_profile.name, windows.clone(), hardware.clone());

            // 1. Capture Pre-Flight Benchmark
            println!(
                "\n{}",
                "[1/4] Capturing pre-flight performance baseline...".yellow()
            );
            let bench_before = benchmark::BenchmarkMetrics::capture("pre-apply");
            bench_before.print_summary();

            // 2. Apply Modules Based on Profile
            println!(
                "\n{}",
                "[2/4] Executing configuration changes safely...".yellow()
            );
            if opt_profile.enable_privacy {
                PrivacyModule::apply(dry_run, &mut snap)?;
            }
            if opt_profile.enable_gaming {
                GamingModule::apply(dry_run, &mut snap)?;
            }
            if opt_profile.enable_ai_doctor {
                AiModule::apply()?;
            }
            if opt_profile.enable_developer {
                DeveloperModule::apply()?;
            }

            // 3. Save Atomic Snapshot
            if !dry_run {
                let saved_snap_path = snap.save(&exe_dir().join("obsidian-state"))?;
                println!(
                    "{} {}",
                    "  [OK] Atomic rollback snapshot saved:".green(),
                    saved_snap_path.display()
                );
            }

            // 4. Post-Flight Health Validation
            println!(
                "\n{}",
                "[3/4] Running post-flight zero-breakage validation...".yellow()
            );
            let checks = ValidationEngine::run_all()?;

            // 5. Generate Report
            println!(
                "\n{}",
                "[4/4] Generating comprehensive execution report...".yellow()
            );
            let report = reporting::ComprehensiveReport {
                timestamp: chrono::Utc::now().to_rfc3339(),
                windows: windows.clone(),
                hardware: hardware.clone(),
                applied_profile: opt_profile.name,
                validation_checks: checks,
                benchmark: Some(bench_before),
            };
            report.save_all(&exe_dir().join("report.md"))?;
            println!("{}", "  [OK] Saved report.md and report.json".green());

            println!(
                "\n{}",
                "================================================================================"
                    .cyan()
            );
            println!(
                "{}",
                "[V] PROJECT OBSIDIAN APPLIED SUCCESSFULLY.".green().bold()
            );
            println!("{}", "    System is optimized for low latency, zero telemetry bloat, and peak stability.".white());
            println!(
                "{}",
                "================================================================================"
                    .cyan()
            );
        }

        Commands::Export { output } => {
            let checks = ValidationEngine::run_all().unwrap_or_default();
            let bench = Some(benchmark::BenchmarkMetrics::capture("export"));
            let report = reporting::ComprehensiveReport {
                timestamp: chrono::Utc::now().to_rfc3339(),
                windows: windows.clone(),
                hardware: hardware.clone(),
                applied_profile: "Audit/Export".to_string(),
                validation_checks: checks,
                benchmark: bench,
            };
            report.save_all(&output)?;
            println!(
                "{} {}",
                "[V] Diagnostic report successfully written to:".green(),
                output.display()
            );
        }
    }
    Ok(())
}

fn run_interactive_menu(windows: &WindowsInfo, hardware: &HardwareInfo) -> Result<()> {
    use std::io::{self, Write};

    loop {
        print_system_overview(windows, hardware);
        println!("{}", "SELECT AN ACTION:".yellow().bold());
        println!(
            "  {} Analyze System (Dry-run, zero changes)",
            "[1]".cyan().bold()
        );
        println!(
            "  {} AI & Gaming Doctor (Inspect CUDA, WSL2, Docker, Anticheats)",
            "[2]".cyan().bold()
        );
        println!(
            "  {} System Benchmark (Measure RAM, CPU, active processes)",
            "[3]".cyan().bold()
        );
        println!(
            "  {} Apply Profile: ULTIMATE (All safe privacy & gaming optimizations)",
            "[4]".green().bold()
        );
        println!(
            "  {} Apply Profile: PRIVACY (Telemetry & ad blocking only)",
            "[5]".green().bold()
        );
        println!(
            "  {} Apply Profile: GAMING (Latency & Game DVR only)",
            "[6]".green().bold()
        );
        println!(
            "  {} Validate System Health (Post-flight zero-breakage check)",
            "[7]".cyan().bold()
        );
        println!(
            "  {} Restore / Rollback (Revert system to exact prior state)",
            "[8]".magenta().bold()
        );
        if !windows.is_admin {
            println!(
                "  {} Elevate to Administrator (Trigger UAC Prompt)",
                "[0]".yellow().bold()
            );
        }
        println!("  {} Exit Project Obsidian", "[9]".dimmed());
        print!("\n{}", "Enter option: ".white().bold());
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let choice = input.trim();

        println!("\n");
        match choice {
            "0" => {
                println!(
                    "{}",
                    "[*] Requesting Administrator elevation via UAC...".cyan()
                );
                WindowsInfo::relaunch_as_admin()?;
            }
            "1" => {
                execute_command(Commands::Analyze, windows, hardware)?;
            }
            "2" => {
                execute_command(Commands::Doctor, windows, hardware)?;
            }
            "3" => {
                execute_command(
                    Commands::Benchmark {
                        label: "manual".to_string(),
                    },
                    windows,
                    hardware,
                )?;
            }
            "4" => {
                execute_command(
                    Commands::Apply {
                        profile: cli::ProfileType::Ultimate,
                        dry_run: false,
                    },
                    windows,
                    hardware,
                )?;
            }
            "5" => {
                execute_command(
                    Commands::Apply {
                        profile: cli::ProfileType::Privacy,
                        dry_run: false,
                    },
                    windows,
                    hardware,
                )?;
            }
            "6" => {
                execute_command(
                    Commands::Apply {
                        profile: cli::ProfileType::Gaming,
                        dry_run: false,
                    },
                    windows,
                    hardware,
                )?;
            }
            "7" => {
                execute_command(Commands::Validate, windows, hardware)?;
            }
            "8" => {
                execute_command(Commands::Restore { snapshot: None }, windows, hardware)?;
            }
            "9" | "q" | "exit" => {
                println!(
                    "{}",
                    "Exiting Project Obsidian. Stay fast, stay private!".cyan()
                );
                break;
            }
            _ => {
                println!(
                    "{}",
                    "[!] Invalid selection. Please enter a valid option number.".red()
                );
            }
        }

        println!("\n{}", "Press Enter to return to menu...".yellow());
        let mut pause = String::new();
        io::stdin().read_line(&mut pause)?;
        print_banner();
    }

    Ok(())
}

fn print_banner() {
    let version = env!("CARGO_PKG_VERSION");
    println!(
        "{}",
        "================================================================================".cyan()
    );
    println!(
        "{}",
        "         _____     _           _      ____  _       _     _             ".cyan()
    );
    println!(
        "{}",
        "        |  _  |___|_|___ ___ _| |_   |    \\| |_ ___|_|___| |___ ___     ".cyan()
    );
    println!(
        "{}",
        "        |   __|  _| | -_|  _| . |    |  |  | . |_ -| | . | | .'|   |    ".cyan()
    );
    println!(
        "{}",
        "        |__|  |_| |_|___|___|___|    |____/|___|___|_|___|_|__,|_|_|    ".cyan()
    );
    println!(
        "{}",
        "                                                                        ".cyan()
    );
    println!(
        "{}",
        "                 Forge Windows into a Privacy-First Workstation                 "
            .white()
            .bold()
    );
    println!(
        "{}",
        "              Zero Placebos • Anti-Cheat Safe • Guaranteed Rollback             ".dimmed()
    );
    println!(
        "                               {}                                    ",
        format!("v{}  |  github.com/Nheoz/project-obsidian", version).dimmed()
    );
    println!(
        "{}",
        "================================================================================".cyan()
    );
}

fn print_system_overview(win: &WindowsInfo, hw: &HardwareInfo) {
    println!("{}", "--- [System Architecture & Baseline] ---".yellow());
    println!(
        "  {:<25} : {} (Build {})",
        "Operating System", win.caption, win.build_number
    );
    println!("  {:<25} : {}", "Processor", hw.cpu_brand);
    println!("  {:<25} : {:.2} GB", "System Memory", hw.total_memory_gb);
    for gpu in &hw.gpus {
        println!(
            "  {:<25} : {} [Driver: {}]",
            "Graphics Adapter", gpu.name, gpu.driver_version
        );
    }
    println!(
        "  {:<25} : {}",
        "Privileges",
        if win.is_admin {
            "Administrator [Ready]".green()
        } else {
            "Standard User [Elevation required to apply]".yellow()
        }
    );
    println!();
}
