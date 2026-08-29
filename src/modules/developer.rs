use anyhow::Result;
use colored::*;
use std::process::Command;

pub struct DeveloperModule;

impl DeveloperModule {
    pub fn audit() -> Result<()> {
        println!("{}", "--- [Developer Tooling Audit] ---".yellow());
        let tools = [
            ("Git", "git", "--version"),
            ("Windows Terminal", "wt", "-v"),
            ("VS Code", "code", "--version"),
            ("Winget Package Manager", "winget", "--version"),
        ];

        for (name, bin, arg) in tools {
            let out = Command::new(bin).arg(arg).output();
            match out {
                Ok(o) if o.status.success() => {
                    let first_line = String::from_utf8_lossy(&o.stdout)
                        .lines()
                        .next()
                        .unwrap_or("Installed")
                        .to_string();
                    println!("  {:<25} : {}", name, first_line.green());
                }
                _ => {
                    println!("  {:<25} : {}", name, "Not installed or not in PATH".yellow());
                }
            }
        }
        Ok(())
    }
}
