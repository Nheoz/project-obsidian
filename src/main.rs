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
use std::path::PathBuf;
use validation::ValidationEngine;
use windows::WindowsInfo;

/// Returns the directory containing the running executable.
fn exe_dir() -> PathBuf {
    std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|d| d.to_path_buf()))
        .unwrap_or_else(|| PathBuf::from("."))
}

/// Initialize the tracing logger: always writes DEBUG+ to a log file next to the exe.
fn init_logging() {
    use tracing_subscriber::fmt;
    use tracing_subscriber::prelude::*;

    let log_dir = exe_dir();
    let log_file = log_dir.join("obsidian.log");

    // File appender — keeps a persistent trace of every run
    if let Ok(file) = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&log_file)
    {
        let file_layer = fmt::layer()
            .with_writer(move || {
                file.try_clone()
                    .unwrap_or_else(|_| std::fs::File::create("/dev/null").unwrap())
            })
            .with_ansi(false)
            .with_target(false);

        let _ = tracing_subscriber::registry().with(file_layer).try_init();
    }
}

fn main() -> Result<()> {
    #[cfg(windows)]
    let _ = colored::control::set_virtual_terminal(true);

    init_logging();

    let args = Cli::parse();

    // If launched with no subcommands (double-clicked in Explorer without admin):
    // Relaunch inside a dedicated elevated cmd.exe console window
    if args.command.is_none() && !args.interactive_terminal {
        if !WindowsInfo::check_is_admin() {
            let current_exe = std::env::current_exe()?;
            let script = format!(
                "Start-Process cmd.exe -ArgumentList '/c \"\"{}\"\" --interactive-terminal' -Verb RunAs",
                current_exe.display()
            );
            let spawn_result = std::process::Command::new("powershell")
                .args(["-NoProfile", "-Command", &script])
                .spawn();
            if spawn_result.is_err() {
                eprintln!("{}", "[!] Failed to launch elevated console. Please run obsidian.exe as Administrator manually.".red());
            }
            return Ok(());
        }
    }

    // --json: suppress colored output, emit JSON where applicable
    let json_mode = args.json;
    // --verbose: pass-through stdout from PowerShell subprocesses
    let _verbose = args.verbose;

    if !json_mode {
        print_banner();
    }

    let windows = WindowsInfo::detect();
    let hardware = HardwareInfo::detect();

    if let Some(cmd) = args.command {
        execute_command(cmd, &windows, &hardware, json_mode)?;
    } else {
        run_interactive_menu(&windows, &hardware)?;
    }

    Ok(())
}

fn execute_command(
    cmd: Commands,
    windows: &WindowsInfo,
    hardware: &HardwareInfo,
    json_mode: bool,
) -> Result<()> {
    match cmd {
        Commands::Status => {
            if json_mode {
                let val = serde_json::json!({
                    "os": windows.caption,
                    "build": windows.build_number,
                    "edition": windows.edition,
                    "is_admin": windows.is_admin,
                    "cpu": hardware.cpu_brand,
                    "ram_gb": hardware.total_memory_gb,
                });
                println!("{}", serde_json::to_string_pretty(&val)?);
                return Ok(());
            }
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
            println!("{}", "  [*] Collecting system metrics...".dimmed());
            let metrics = benchmark::BenchmarkMetrics::capture(&label);
            if json_mode {
                println!("{}", serde_json::to_string_pretty(&metrics)?);
            } else {
                metrics.print_summary();
            }
            let out_path = exe_dir().join(format!("benchmark-{}.json", label));
            metrics.save_to_file(&out_path)?;
            println!(
                "  [OK] Saved benchmark metrics to: {}",
                out_path.display().to_string().green()
            );
        }

        Commands::Validate => {
            ValidationEngine::run_all()?;
        }

        Commands::Restore { snapshot } => {
            RollbackEngine::execute(&exe_dir().join("obsidian-state"), snapshot)?;
        }

        Commands::Apply { profile, dry_run } => {
            let opt_profile = OptimizationProfile::from_type(profile);
            tracing::info!(profile = %opt_profile.name, dry_run, "Applying profile");

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

            // ── Confirmation prompt (skip in dry-run or non-interactive) ──────────
            if !dry_run {
                use std::io::{self, Write};
                println!();
                println!(
                    "{}",
                    "  ┌─────────────────────────────────────────────────────────┐".yellow()
                );
                println!(
                    "{}",
                    "  │  ⚠  This will apply permanent changes to your system.   │"
                        .yellow()
                        .bold()
                );
                println!(
                    "{}",
                    "  │     A rollback snapshot will be saved automatically.     │".yellow()
                );
                println!(
                    "{}",
                    "  └─────────────────────────────────────────────────────────┘".yellow()
                );
                print!(
                    "\n  Type {} to confirm, or press Enter to cancel: ",
                    "YES".green().bold()
                );
                io::stdout().flush()?;
                let mut confirm = String::new();
                io::stdin().read_line(&mut confirm)?;
                if confirm.trim() != "YES" {
                    println!("{}", "\n  [!] Cancelled. Zero changes applied.".yellow());
                    return Ok(());
                }
                println!();
            }

            let mut snap = Snapshot::new(&opt_profile.name, windows.clone(), hardware.clone());

            // 1. Pre-Flight Benchmark
            println!(
                "\n{}",
                "[1/5] Capturing pre-flight performance baseline...".yellow()
            );
            let bench_before = benchmark::BenchmarkMetrics::capture("pre-apply");
            bench_before.print_summary();

            // 2. Apply Modules
            println!(
                "\n{}",
                "[2/5] Executing configuration changes safely...".yellow()
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
                "[3/5] Running post-flight zero-breakage validation...".yellow()
            );
            let checks = ValidationEngine::run_all()?;

            // 5. Post-Flight Benchmark + Delta Comparison
            println!(
                "\n{}",
                "[4/5] Capturing post-apply performance metrics...".yellow()
            );
            let bench_after = benchmark::BenchmarkMetrics::capture("post-apply");
            benchmark::BenchmarkMetrics::print_comparison(&bench_before, &bench_after);

            // 6. Generate Report
            println!(
                "\n{}",
                "[5/5] Generating comprehensive execution report...".yellow()
            );
            let compare_md = benchmark::BenchmarkMetrics::compare(&bench_before, &bench_after);
            let report = reporting::ComprehensiveReport {
                timestamp: chrono::Utc::now().to_rfc3339(),
                windows: windows.clone(),
                hardware: hardware.clone(),
                applied_profile: opt_profile.name.clone(),
                validation_checks: checks,
                benchmark: Some(bench_after),
            };
            let report_path = exe_dir().join("report.md");
            report.save_all(&report_path)?;
            // Append the benchmark delta to the report
            let _ = std::fs::OpenOptions::new()
                .append(true)
                .open(&report_path)
                .and_then(|mut f| {
                    use std::io::Write;
                    writeln!(f, "\n\n---\n\n{}", compare_md)
                });
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
            println!(
                "{}",
                "    System is optimized for low latency, zero telemetry bloat, and peak stability.".white()
            );
            println!(
                "{}",
                "================================================================================"
                    .cyan()
            );

            tracing::info!(profile = %opt_profile.name, "Profile applied successfully");
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
            if json_mode {
                let json = serde_json::to_string_pretty(&serde_json::json!({
                    "timestamp": report.timestamp,
                    "profile": report.applied_profile,
                    "os": report.windows.caption,
                    "build": report.windows.build_number,
                }))?;
                println!("{}", json);
            } else {
                report.save_all(&output)?;
                println!(
                    "{} {}",
                    "[V] Diagnostic report successfully written to:".green(),
                    output.display()
                );
            }
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
            "  {} Apply Profile: ULTIMATE  (Privacy + Gaming + AI + Developer)",
            "[4]".green().bold()
        );
        println!(
            "  {} Apply Profile: PRIVACY   (Telemetry & ad blocking only)",
            "[5]".green().bold()
        );
        println!(
            "  {} Apply Profile: GAMING    (Latency & Game DVR only)",
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
                execute_command(Commands::Analyze, windows, hardware, false)?;
            }
            "2" => {
                execute_command(Commands::Doctor, windows, hardware, false)?;
            }
            "3" => {
                execute_command(
                    Commands::Benchmark {
                        label: "manual".to_string(),
                    },
                    windows,
                    hardware,
                    false,
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
                    false,
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
                    false,
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
                    false,
                )?;
            }
            "7" => {
                execute_command(Commands::Validate, windows, hardware, false)?;
            }
            "8" => {
                execute_command(
                    Commands::Restore { snapshot: None },
                    windows,
                    hardware,
                    false,
                )?;
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
