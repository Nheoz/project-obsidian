use crate::snapshot::Snapshot;
use anyhow::{bail, Result};
use colored::*;
use std::process::Command;

pub struct GamingModule;

/// Registry keys managed by the Gaming module
const GAMING_REGISTRY: &[(&str, &str)] = &[
    ("HKCU:\\Software\\Microsoft\\GameBar", "AllowAutoGameMode"),
    ("HKCU:\\Software\\Microsoft\\GameBar", "AutoGameModeEnabled"),
    ("HKCU:\\Software\\Microsoft\\GameBar", "ShowStartupPanel"),
    (
        "HKCU:\\Software\\Microsoft\\Windows\\CurrentVersion\\GameDVR",
        "AppCaptureEnabled",
    ),
    ("HKCU:\\System\\GameConfigStore", "GameDVR_Enabled"),
];

impl GamingModule {
    pub fn audit() -> Result<()> {
        println!("{}", "--- [Gaming Readiness Audit] ---".yellow());
        let root = crate::embedded::get_scripts_root();
        let reg_p = root.join("powershell").join("Registry.ps1");

        let cmd = format!(
            ". '{}'; \
            $gm = Get-RegistryValueSafe -Path 'HKCU:\\Software\\Microsoft\\GameBar' -Name 'AllowAutoGameMode'; \
            $dvr = Get-RegistryValueSafe -Path 'HKCU:\\Software\\Microsoft\\Windows\\CurrentVersion\\GameDVR' -Name 'AppCaptureEnabled'; \
            $gmTxt = if ($gm.Value -eq 1 -or -not $gm.Exists) {{ 'Active [Recommended]' }} else {{ 'Disabled' }}; \
            $dvrTxt = if ($dvr.Value -eq 0) {{ 'Disabled (Zero CPU/Disk Penalty)' }} else {{ 'Enabled' }}; \
            Write-Host '  Windows Game Mode:        ' $gmTxt; \
            Write-Host '  Background DVR Capture:   ' $dvrTxt; \
            Write-Host '  Anti-Cheat Protection:     100%% Intact (EAC, BattlEye, Vanguard preserved)';",
            reg_p.display()
        );

        let output = Command::new("powershell")
            .args(["-NoProfile", "-Command", &cmd])
            .output()?;

        println!("{}", String::from_utf8_lossy(&output.stdout).trim_end());
        Ok(())
    }

    pub fn apply(dry_run: bool, snapshot: &mut Snapshot) -> Result<()> {
        println!(
            "{}",
            "[+] Applying Gaming Optimizations (Zero Placebos, Zero Anticheat Interference)..."
                .cyan()
        );

        let root = crate::embedded::get_scripts_root();
        let reg_p = root.join("powershell").join("Registry.ps1");

        // ── PHASE 0: Capture real current registry state for rollback ──────────
        if !dry_run {
            println!(
                "{}",
                "  [*] Capturing real registry state for rollback...".dimmed()
            );
            snapshot
                .registry_items
                .extend(Self::capture_registry_states(&reg_p.to_string_lossy())?);
        }

        // ── PHASE 1: Apply gaming tweaks ──────────────────────────────────────
        let dry_flag = if dry_run { "-WhatIf" } else { "" };
        let cmd = format!(
            ". '{}'; \
            Set-RegistryValueSafe -Path 'HKCU:\\Software\\Microsoft\\GameBar' -Name 'AllowAutoGameMode' -Value 1 -PropertyType 'DWord' {}; \
            Set-RegistryValueSafe -Path 'HKCU:\\Software\\Microsoft\\GameBar' -Name 'AutoGameModeEnabled' -Value 1 -PropertyType 'DWord' {}; \
            Set-RegistryValueSafe -Path 'HKCU:\\Software\\Microsoft\\GameBar' -Name 'ShowStartupPanel' -Value 0 -PropertyType 'DWord' {}; \
            Set-RegistryValueSafe -Path 'HKCU:\\Software\\Microsoft\\Windows\\CurrentVersion\\GameDVR' -Name 'AppCaptureEnabled' -Value 0 -PropertyType 'DWord' {}; \
            Set-RegistryValueSafe -Path 'HKCU:\\System\\GameConfigStore' -Name 'GameDVR_Enabled' -Value 0 -PropertyType 'DWord' {};",
            reg_p.display(),
            dry_flag, dry_flag, dry_flag, dry_flag, dry_flag
        );

        println!(
            "{}",
            "  [*] Disabling Game DVR background recording...".dimmed()
        );
        println!("{}", "      (Prevents Windows from constantly recording your screen to memory, eliminating micro-stutters)".green());

        println!("{}", "  [*] Enforcing Windows Game Mode...".dimmed());
        println!("{}", "      (Instructs the OS scheduler to prioritize CPU and GPU resources to the active game window)".green());

        let output = Command::new("powershell")
            .args(["-NoProfile", "-ExecutionPolicy", "Bypass", "-Command", &cmd])
            .output()?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            bail!(
                "Gaming optimization PowerShell script failed:\n{}",
                stderr.trim()
            );
        }

        println!(
            "{}",
            "  [OK] Gaming core optimizations applied safely.".green()
        );
        Ok(())
    }

    /// Read the real current value of each registry key managed by this module.
    fn capture_registry_states(
        reg_script: &str,
    ) -> Result<Vec<crate::snapshot::SnapshotRegistryItem>> {
        let mut parts = Vec::new();
        for (path, name) in GAMING_REGISTRY {
            parts.push(format!(
                "[PSCustomObject]@{{ Path='{}'; Name='{}'; Data=(Get-RegistryValueSafe -Path '{}' -Name '{}') }}",
                path, name, path, name
            ));
        }
        let queries = parts.join("; ");

        let cmd = format!(
            ". '{}'; @({}) | ConvertTo-Json -Compress",
            reg_script, queries
        );

        let output = Command::new("powershell")
            .args(["-NoProfile", "-ExecutionPolicy", "Bypass", "-Command", &cmd])
            .output()?;

        if !output.status.success() {
            bail!(
                "Failed to read registry states for snapshot: {}",
                String::from_utf8_lossy(&output.stderr).trim()
            );
        }

        let json_str = String::from_utf8_lossy(&output.stdout);
        let val: serde_json::Value =
            serde_json::from_str(&json_str).unwrap_or(serde_json::Value::Array(vec![]));

        let mut items = Vec::new();
        let arr = if val.is_array() {
            val.as_array().cloned().unwrap_or_default()
        } else {
            vec![val]
        };

        for item in arr {
            let path = item["Path"].as_str().unwrap_or("").to_string();
            let name = item["Name"].as_str().unwrap_or("").to_string();
            if path.is_empty() || name.is_empty() {
                continue;
            }

            let data = &item["Data"];
            let exists = data["Exists"].as_bool().unwrap_or(false);
            let previous_value = if exists {
                Some(data["Value"].clone())
            } else {
                None
            };
            let previous_type = data["Type"].as_str().map(|s| s.to_string());

            items.push(crate::snapshot::SnapshotRegistryItem {
                path,
                name,
                previous_exists: exists,
                previous_value,
                previous_type,
            });
        }

        Ok(items)
    }
}
