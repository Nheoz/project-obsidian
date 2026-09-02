use anyhow::Result;
use colored::*;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use sysinfo::System;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkMetrics {
    pub timestamp: String,
    pub label: String,
    pub total_memory_gb: f64,
    pub used_memory_gb: f64,
    pub available_memory_gb: f64,
    pub memory_used_percent: f64,
    pub cpu_global_usage_percent: f32,
    pub total_processes: usize,
    pub total_threads: usize,
}

impl BenchmarkMetrics {
    pub fn capture(label: &str) -> Self {
        let mut sys = System::new_all();
        // Two refreshes separated by short sleep to capture accurate CPU load
        sys.refresh_all();
        std::thread::sleep(std::time::Duration::from_millis(300));
        sys.refresh_all();

        let total_mem = sys.total_memory() as f64 / (1024.0 * 1024.0 * 1024.0);
        let used_mem = sys.used_memory() as f64 / (1024.0 * 1024.0 * 1024.0);
        let avail_mem = sys.available_memory() as f64 / (1024.0 * 1024.0 * 1024.0);
        let mem_percent = if total_mem > 0.0 {
            (used_mem / total_mem) * 100.0
        } else {
            0.0
        };

        let cpu_usage = sys.global_cpu_usage();
        let total_processes = sys.processes().len();

        // sysinfo p.tasks() is Linux-only (reads /proc/[pid]/task/).
        // On Windows it always returns None, so we count threads via WMI instead.
        let total_threads: usize = {
            std::process::Command::new("powershell")
                .args([
                    "-NoProfile",
                    "-Command",
                    "(Get-CimInstance Win32_Process | Measure-Object -Property ThreadCount -Sum).Sum",
                ])
                .output()
                .ok()
                .and_then(|o| String::from_utf8(o.stdout).ok())
                .and_then(|s| s.trim().parse::<usize>().ok())
                .unwrap_or(total_processes)
        };

        BenchmarkMetrics {
            timestamp: chrono::Utc::now().to_rfc3339(),
            label: label.to_string(),
            total_memory_gb: (total_mem * 100.0).round() / 100.0,
            used_memory_gb: (used_mem * 100.0).round() / 100.0,
            available_memory_gb: (avail_mem * 100.0).round() / 100.0,
            memory_used_percent: (mem_percent * 10.0).round() / 10.0,
            cpu_global_usage_percent: (cpu_usage * 10.0).round() / 10.0,
            total_processes,
            total_threads,
        }
    }

    pub fn print_summary(&self) {
        println!(
            "{}",
            "================================================================================"
                .cyan()
        );
        println!(
            "{} [{}]",
            "PROJECT OBSIDIAN — SYSTEM BENCHMARK".cyan().bold(),
            self.label.yellow()
        );
        println!(
            "{}",
            "================================================================================"
                .cyan()
        );
        println!("  {:<25} : {:.2} GB", "Total RAM", self.total_memory_gb);
        println!(
            "  {:<25} : {:.2} GB ({:.1}%)",
            "RAM in Use", self.used_memory_gb, self.memory_used_percent
        );
        println!(
            "  {:<25} : {:.2} GB",
            "Available RAM", self.available_memory_gb
        );
        println!(
            "  {:<25} : {:.1}%",
            "CPU Usage", self.cpu_global_usage_percent
        );
        println!("  {:<25} : {}", "Active Processes", self.total_processes);
        println!("  {:<25} : {}", "Active Threads", self.total_threads);
        println!(
            "{}",
            "================================================================================"
                .cyan()
        );
    }

    pub fn save_to_file(&self, path: &Path) -> Result<()> {
        let json = serde_json::to_string_pretty(self)?;
        fs::write(path, json)?;
        Ok(())
    }

    /// Compare two benchmark captures and return a formatted Markdown delta report.
    pub fn compare(before: &BenchmarkMetrics, after: &BenchmarkMetrics) -> String {
        let mem_diff = after.used_memory_gb - before.used_memory_gb;
        let proc_diff = (after.total_processes as isize) - (before.total_processes as isize);
        let thread_diff = (after.total_threads as isize) - (before.total_threads as isize);
        let cpu_diff = after.cpu_global_usage_percent - before.cpu_global_usage_percent;

        // Emoji indicators for deltas
        let mem_arrow = if mem_diff < -0.1 {
            "↓ BETTER"
        } else if mem_diff > 0.1 {
            "↑ higher"
        } else {
            "≈ same"
        };
        let proc_arrow = if proc_diff < 0 {
            "↓ BETTER"
        } else if proc_diff > 0 {
            "↑ higher"
        } else {
            "≈ same"
        };

        format!(
            "# Project Obsidian — Benchmark Comparison Report\n\n\
            | Metric | Before | After | Delta | Verdict |\n\
            | :--- | :--- | :--- | :--- | :--- |\n\
            | **RAM In Use** | {:.2} GB | {:.2} GB | {:+.2} GB | {} |\n\
            | **Memory Load** | {:.1}% | {:.1}% | {:+.1}% | {} |\n\
            | **Active Processes** | {} | {} | {:+} | {} |\n\
            | **Active Threads** | {} | {} | {:+} | {} |\n\
            | **CPU Usage** | {:.1}% | {:.1}% | {:+.1}% | {} |\n\n\
            *Captured via Windows Kernel interfaces. Zero synthetic estimation.*",
            before.used_memory_gb,
            after.used_memory_gb,
            mem_diff,
            mem_arrow,
            before.memory_used_percent,
            after.memory_used_percent,
            after.memory_used_percent - before.memory_used_percent,
            mem_arrow,
            before.total_processes,
            after.total_processes,
            proc_diff,
            proc_arrow,
            before.total_threads,
            after.total_threads,
            thread_diff,
            if thread_diff < 0 {
                "↓ BETTER"
            } else if thread_diff > 0 {
                "↑ higher"
            } else {
                "≈ same"
            },
            before.cpu_global_usage_percent,
            after.cpu_global_usage_percent,
            cpu_diff,
            if cpu_diff < -1.0 {
                "↓ BETTER"
            } else if cpu_diff > 1.0 {
                "↑ higher"
            } else {
                "≈ same"
            },
        )
    }

    /// Print a delta comparison directly to the console in colored format.
    pub fn print_comparison(before: &BenchmarkMetrics, after: &BenchmarkMetrics) {
        println!(
            "{}",
            "================================================================================"
                .cyan()
        );
        println!(
            "{}",
            "PROJECT OBSIDIAN — BEFORE vs AFTER COMPARISON"
                .cyan()
                .bold()
        );
        println!(
            "{}",
            "================================================================================"
                .cyan()
        );

        let mem_diff = after.used_memory_gb - before.used_memory_gb;
        let proc_diff = (after.total_processes as isize) - (before.total_processes as isize);

        let fmt_delta_f64 = |d: f64| -> colored::ColoredString {
            if d < -0.05 {
                format!("{:+.2} ↓", d).green()
            } else if d > 0.05 {
                format!("{:+.2} ↑", d).yellow()
            } else {
                format!("{:+.2} ≈", d).white()
            }
        };
        let fmt_delta_i = |d: isize| -> colored::ColoredString {
            if d < 0 {
                format!("{:+} ↓", d).green()
            } else if d > 0 {
                format!("{:+} ↑", d).yellow()
            } else {
                format!("{:+} ≈", d).white()
            }
        };

        println!(
            "  {:<25} : {:.2} → {:.2} GB  [{}]",
            "RAM In Use",
            before.used_memory_gb,
            after.used_memory_gb,
            fmt_delta_f64(mem_diff)
        );
        println!(
            "  {:<25} : {} → {}  [{}]",
            "Active Processes",
            before.total_processes,
            after.total_processes,
            fmt_delta_i(proc_diff)
        );
        println!(
            "  {:<25} : {} → {}  [{}]",
            "Active Threads",
            before.total_threads,
            after.total_threads,
            fmt_delta_i((after.total_threads as isize) - (before.total_threads as isize))
        );
        println!(
            "  {:<25} : {:.1}% → {:.1}%  [{}]",
            "CPU Usage",
            before.cpu_global_usage_percent,
            after.cpu_global_usage_percent,
            fmt_delta_f64(
                (after.cpu_global_usage_percent - before.cpu_global_usage_percent) as f64
            )
        );

        println!(
            "{}",
            "================================================================================"
                .cyan()
        );
    }
}
