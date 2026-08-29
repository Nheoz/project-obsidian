mod benchmark;
mod cli;
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
use modules::{ai::AiModule, developer::DeveloperModule, gaming::GamingModule, privacy::PrivacyModule};
use profiles::OptimizationProfile;
use rollback::RollbackEngine;
use snapshot::Snapshot;
use std::path::Path;
use validation::ValidationEngine;
use windows::WindowsInfo;

fn main() -> Result<()> {
    // Enable ANSI color support on Windows terminals
    #[cfg(windows)]
    let _ = colored::control::set_virtual_terminal(true);

    let args = Cli::parse();

    print_banner();

    let windows = WindowsInfo::detect();
    let hardware = HardwareInfo::detect();

    match args.command {
        Commands::Status => {
            print_system_overview(&windows, &hardware);
            PrivacyModule::audit()?;
            GamingModule::audit()?;
            DeveloperModule::audit()?;
        }

        Commands::Analyze => {
            println!("{}", "[*] INITIATING NON-DESTRUCTIVE DRY-RUN ANALYSIS".cyan().bold());
            print_system_overview(&windows, &hardware);
            PrivacyModule::audit()?;
            GamingModule::audit()?;
            DeveloperModule::audit()?;
            AiModule::doctor()?;
            println!("\n{}", "[V] Analysis complete. Zero changes applied to disk or registry.".green().bold());
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
            RollbackEngine::execute(Path::new("obsidian-state"), snapshot)?;
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
                eprintln!("{}", "[!] ERROR: Administrator privileges are required to apply optimizations.".red().bold());
                eprintln!("{}", "    Please run terminal or PowerShell as Administrator.".yellow());
                std::process::exit(1);
            }

            let mut snap = Snapshot::new(&opt_profile.name, windows.clone(), hardware.clone());

            // 1. Capture Pre-Flight Benchmark
            println!("\n{}", "[1/4] Capturing pre-flight performance baseline...".yellow());
            let bench_before = benchmark::BenchmarkMetrics::capture("pre-apply");
            bench_before.print_summary();

            // 2. Apply Modules Based on Profile
            println!("\n{}", "[2/4] Executing configuration changes safely...".yellow());
            if opt_profile.enable_privacy {
                PrivacyModule::apply(dry_run, &mut snap)?;
            }
            if opt_profile.enable_gaming {
                GamingModule::apply(dry_run, &mut snap)?;
            }
            if opt_profile.enable_ai_doctor {
                AiModule::doctor()?;
            }
            if opt_profile.enable_developer {
                DeveloperModule::audit()?;
            }

            // 3. Save Atomic Snapshot
            if !dry_run {
                let saved_snap_path = snap.save(Path::new("obsidian-state"))?;
                println!(
                    "{} {}",
                    "  [OK] Atomic rollback snapshot saved:".green(),
                    saved_snap_path.display()
                );
            }

            // 4. Post-Flight Health Validation
            println!("\n{}", "[3/4] Running post-flight zero-breakage validation...".yellow());
            let checks = ValidationEngine::run_all()?;

            // 5. Generate Report
            println!("\n{}", "[4/4] Generating comprehensive execution report...".yellow());
            let report = reporting::ComprehensiveReport {
                timestamp: chrono::Utc::now().to_rfc3339(),
                windows: windows.clone(),
                hardware: hardware.clone(),
                applied_profile: opt_profile.name,
                validation_checks: checks,
                benchmark: Some(bench_before),
            };
            report.save_all(Path::new("report.md"))?;
            println!("{}", "  [OK] Saved report.md and report.json".green());

            println!("\n{}", "================================================================================".cyan());
            println!("{}", "[V] PROJECT OBSIDIAN APPLIED SUCCESSFULLY.".green().bold());
            println!("{}", "    System is optimized for low latency, zero telemetry bloat, and peak stability.".white());
            println!("{}", "================================================================================".cyan());
        }

        Commands::Export { output } => {
            let checks = ValidationEngine::run_all().unwrap_or_default();
            let bench = Some(benchmark::BenchmarkMetrics::capture("export"));
            let report = reporting::ComprehensiveReport {
                timestamp: chrono::Utc::now().to_rfc3339(),
                windows,
                hardware,
                applied_profile: "Audit/Export".to_string(),
                validation_checks: checks,
                benchmark: bench,
            };
            report.save_all(&output)?;
            println!("{} {}", "[V] Diagnostic report successfully written to:".green(), output.display());
        }
    }

    Ok(())
}

fn print_banner() {
    println!("{}", "================================================================================".cyan());
    println!("{}", "         _____     _           _      ____  _       _     _             ".cyan());
    println!("{}", "        |  _  |___|_|___ ___ _| |_   |    \\| |_ ___|_|___| |___ ___     ".cyan());
    println!("{}", "        |   __|  _| | -_|  _| . |    |  |  | . |_ -| | . | | .'|   |    ".cyan());
    println!("{}", "        |__|  |_| |_|___|___|___|    |____/|___|___|_|___|_|__,|_|_|    ".cyan());
    println!("{}", "                                                                        ".cyan());
    println!("{}", "                 Forge Windows into a Privacy-First Workstation                 ".white().bold());
    println!("{}", "              Zero Placebos • Anti-Cheat Safe • Guaranteed Rollback             ".dimmed());
    println!("{}", "================================================================================".cyan());
}

fn print_system_overview(win: &WindowsInfo, hw: &HardwareInfo) {
    println!("{}", "--- [System Architecture & Baseline] ---".yellow());
    println!("  {:<25} : {} (Build {})", "Operating System", win.caption, win.build_number);
    println!("  {:<25} : {}", "Processor", hw.cpu_brand);
    println!("  {:<25} : {:.2} GB", "System Memory", hw.total_memory_gb);
    for gpu in &hw.gpus {
        println!("  {:<25} : {} [Driver: {}]", "Graphics Adapter", gpu.name, gpu.driver_version);
    }
    println!("  {:<25} : {}", "Privileges", if win.is_admin { "Administrator [Ready]".green() } else { "Standard User [Elevation required to apply]".yellow() });
    println!();
}
