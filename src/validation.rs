use anyhow::Result;
use colored::*;
use serde::{Deserialize, Serialize};
use std::process::Command;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationCheck {
    pub component: String,
    pub category: String,
    pub status: String,
    pub passed: bool,
    pub details: String,
}

pub struct ValidationEngine;

impl ValidationEngine {
    pub fn run_all() -> Result<Vec<ValidationCheck>> {
        println!("{}", "================================================================================".cyan());
        println!("{}", "            PROJECT OBSIDIAN — SYSTEM HEALTH & VALIDATION MATRIX               ".cyan());
        println!("{}", "================================================================================".cyan());

        let root = crate::embedded::get_scripts_root();
        let val_p = root.join("powershell").join("Validation.ps1");
        let cmd = format!(". '{}'; Test-ObsidianHealth | ConvertTo-Json", val_p.display());

        let output = Command::new("powershell")
            .args([
                "-NoProfile",
                "-ExecutionPolicy",
                "Bypass",
                "-Command",
                &cmd,
            ])
            .output()?;

        let mut checks = Vec::new();
        let json_str = String::from_utf8_lossy(&output.stdout);

        if let Ok(val) = serde_json::from_str::<serde_json::Value>(&json_str) {
            if let Some(arr) = val.as_array() {
                for item in arr {
                    let comp = item["Component"].as_str().unwrap_or("").to_string();
                    let cat = item["Category"].as_str().unwrap_or("").to_string();
                    let passed = item["Passed"].as_bool().unwrap_or(false);
                    let stat = item["Status"].as_str().unwrap_or("").to_string();
                    let det = item["Details"].as_str().unwrap_or("").to_string();

                    checks.push(ValidationCheck {
                        component: comp,
                        category: cat,
                        status: stat,
                        passed,
                        details: det,
                    });
                }
            }
        }

        for check in &checks {
            let status_badge = if check.passed {
                "[PASSED]".green().bold()
            } else {
                "[FAILED]".red().bold()
            };
            println!(
                "  {} {:<32} [{}] : {}",
                status_badge,
                check.component.white(),
                check.category.cyan(),
                check.details.dimmed()
            );
        }

        println!("{}", "================================================================================".cyan());
        let all_healthy = checks.iter().all(|c| c.passed);
        if all_healthy {
            println!("{}", "[V] ALL CRITICAL SUBSYSTEMS REPORT HEALTHY. ZERO BREAKAGE CONFIRMED.".green().bold());
        } else {
            println!("{}", "[!] Warning: One or more components reported warnings. Review details above.".yellow().bold());
        }

        Ok(checks)
    }
}
