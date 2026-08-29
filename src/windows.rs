use serde::{Deserialize, Serialize};
use std::process::Command;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowsInfo {
    pub caption: String,
    pub version: String,
    pub build_number: u32,
    pub is_win11: bool,
    pub is_admin: bool,
    pub edition: String,
}

impl WindowsInfo {
    pub fn detect() -> Self {
        let is_admin = Self::check_is_admin();

        let mut caption = "Windows 11".to_string();
        let mut version = "10.0".to_string();
        let mut build_number = 22631;
        let mut edition = "Pro".to_string();

        let output = Command::new("powershell")
            .args([
                "-NoProfile",
                "-Command",
                "(Get-CimInstance Win32_OperatingSystem) | Select-Object Caption, Version, BuildNumber | ConvertTo-Json",
            ])
            .output();

        if let Ok(out) = output {
            let json_str = String::from_utf8_lossy(&out.stdout);
            if let Ok(val) = serde_json::from_str::<serde_json::Value>(&json_str) {
                if let Some(c) = val["Caption"].as_str() { caption = c.to_string(); }
                if let Some(v) = val["Version"].as_str() { version = v.to_string(); }
                if let Some(b) = val["BuildNumber"].as_str() {
                    build_number = b.parse().unwrap_or(22631);
                }
            }
        }

        if caption.to_lowercase().contains("pro") {
            edition = "Pro".to_string();
        } else if caption.to_lowercase().contains("enterprise") {
            edition = "Enterprise".to_string();
        } else if caption.to_lowercase().contains("home") {
            edition = "Home".to_string();
        }

        let is_win11 = build_number >= 22000;

        WindowsInfo {
            caption,
            version,
            build_number,
            is_win11,
            is_admin,
            edition,
        }
    }

    pub fn check_is_admin() -> bool {
        let output = Command::new("powershell")
            .args([
                "-NoProfile",
                "-Command",
                "([Security.Principal.WindowsPrincipal][Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)",
            ])
            .output();

        if let Ok(out) = output {
            let s = String::from_utf8_lossy(&out.stdout).trim().to_lowercase();
            s == "true"
        } else {
            false
        }
    }

    pub fn relaunch_as_admin() -> anyhow::Result<()> {
        let current_exe = std::env::current_exe()?;
        let _ = Command::new("powershell")
            .args([
                "-NoProfile",
                "-Command",
                &format!("Start-Process -FilePath '{}' -Verb RunAs", current_exe.display()),
            ])
            .spawn();
        std::process::exit(0);
    }
}
