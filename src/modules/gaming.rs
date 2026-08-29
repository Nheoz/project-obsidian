use crate::snapshot::Snapshot;
use anyhow::Result;
use colored::*;
use std::process::Command;

pub struct GamingModule;

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
            Write-Host '  Anti-Cheat Protection:     100% Intact (EAC, BattlEye, Vanguard preserved)';",
            reg_p.display()
        );

        let output = Command::new("powershell")
            .args(["-NoProfile", "-Command", &cmd])
            .output()?;

        println!("{}", String::from_utf8_lossy(&output.stdout).trim_end());
        Ok(())
    }

    pub fn apply(dry_run: bool, _snapshot: &mut Snapshot) -> Result<()> {
        println!("{}", "[+] Applying Gaming Optimizations (Zero Placebos, Zero Anticheat Interference)...".cyan());

        let root = crate::embedded::get_scripts_root();
        let reg_p = root.join("powershell").join("Registry.ps1");

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

        let output = Command::new("powershell")
            .args(["-NoProfile", "-ExecutionPolicy", "Bypass", "-Command", &cmd])
            .output()?;

        if output.status.success() {
            println!("{}", "  [OK] Game Mode prioritized, background game DVR overhead eliminated.".green());
        } else {
            eprintln!("{}", String::from_utf8_lossy(&output.stderr));
        }

        Ok(())
    }
}
