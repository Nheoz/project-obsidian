use crate::snapshot::Snapshot;
use anyhow::{Context, Result};
use colored::*;
use std::path::{Path, PathBuf};
use std::process::Command;

pub struct RollbackEngine;

impl RollbackEngine {
    pub fn execute(state_dir: &Path, specific_snapshot: Option<PathBuf>) -> Result<()> {
        println!("{}", "================================================================================".cyan());
        println!("{}", "               PROJECT OBSIDIAN — ATOMIC ROLLBACK ENGINE                        ".cyan());
        println!("{}", "================================================================================".cyan());

        let snapshot_path = match specific_snapshot {
            Some(p) => p,
            None => {
                let latest = Snapshot::load_latest(state_dir)?;
                match latest {
                    Some((path, _)) => path,
                    None => {
                        println!("{}", "[!] No previous snapshot found in obsidian-state/".yellow());
                        println!("{}", "[*] Falling back to default Windows policies restoration...".cyan());
                        Self::execute_powershell_fallback()?;
                        return Ok(());
                    }
                }
            }
        };

        println!("{} {}", "[*] Restoring from snapshot:".cyan(), snapshot_path.display());

        let status = Command::new("powershell")
            .args([
                "-NoProfile",
                "-ExecutionPolicy",
                "Bypass",
                "-File",
                "Restore-Obsidian.ps1",
                "-SnapshotPath",
                snapshot_path.to_str().unwrap_or(""),
            ])
            .status()
            .context("Failed to execute Restore-Obsidian.ps1")?;

        if status.success() {
            println!("{}", "[V] Rollback completed successfully!".green().bold());
        } else {
            eprintln!("{}", "[X] Rollback process returned errors. Review logs for details.".red());
        }

        Ok(())
    }

    fn execute_powershell_fallback() -> Result<()> {
        let status = Command::new("powershell")
            .args([
                "-NoProfile",
                "-ExecutionPolicy",
                "Bypass",
                "-File",
                "Restore-Obsidian.ps1",
            ])
            .status()
            .context("Failed to execute fallback Restore-Obsidian.ps1")?;

        if !status.success() {
            anyhow::bail!("Fallback restoration failed with exit code: {:?}", status.code());
        }
        Ok(())
    }
}
