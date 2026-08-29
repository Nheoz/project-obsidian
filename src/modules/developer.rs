use anyhow::Result;
use colored::*;
use std::process::Command;

pub struct DeveloperModule;

impl DeveloperModule {
    pub fn audit() -> Result<()> {
        println!("{}", "--- [Developer Tooling Audit] ---".yellow());
        // 1. Git
        print!("  {:<25} : ", "Git");
        if let Ok(o) = Command::new("git").arg("--version").output() {
            if o.status.success() {
                let s = String::from_utf8_lossy(&o.stdout).trim().to_string();
                println!("{}", s.green());
            } else {
                println!("{}", "Not found in PATH".yellow());
            }
        } else {
            println!("{}", "Not installed".yellow());
        }

        // 2. Windows Terminal (Silent query without GUI modal dialog)
        print!("  {:<25} : ", "Windows Terminal");
        let wt_out = Command::new("powershell")
            .args(["-NoProfile", "-Command", "(Get-AppxPackage Microsoft.WindowsTerminal* -ErrorAction SilentlyContinue | Select-Object -First 1).Version"])
            .output();
        if let Ok(o) = wt_out {
            let ver = String::from_utf8_lossy(&o.stdout).trim().to_string();
            if !ver.is_empty() {
                println!("{} {}", "v".green(), ver.green());
            } else {
                println!("{}", "Not installed or inbox console used".yellow());
            }
        } else {
            println!("{}", "Not installed".yellow());
        }

        // 3. VS Code
        print!("  {:<25} : ", "VS Code");
        if let Ok(o) = Command::new("code").arg("--version").output() {
            if o.status.success() {
                let s = String::from_utf8_lossy(&o.stdout).lines().next().unwrap_or("Installed").to_string();
                println!("{}", s.green());
            } else {
                println!("{}", "Not found in PATH".yellow());
            }
        } else {
            println!("{}", "Not installed or not in PATH".yellow());
        }

        // 4. Winget Package Manager
        print!("  {:<25} : ", "Winget Package Manager");
        if let Ok(o) = Command::new("winget").arg("--version").output() {
            if o.status.success() {
                let s = String::from_utf8_lossy(&o.stdout).trim().to_string();
                println!("{}", s.green());
            } else {
                println!("{}", "Not found in PATH".yellow());
            }
        } else {
            println!("{}", "Not installed".yellow());
        }
        Ok(())
    }
}
