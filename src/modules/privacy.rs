use crate::snapshot::Snapshot;
use anyhow::Result;
use colored::*;
use std::process::Command;

pub struct PrivacyModule;

impl PrivacyModule {
    pub fn audit() -> Result<()> {
        println!("{}", "--- [Privacy Posture Audit] ---".yellow());
        let output = Command::new("powershell")
            .args([
                "-NoProfile",
                "-Command",
                ". .\\powershell\\Services.ps1; . .\\powershell\\ScheduledTasks.ps1; . .\\powershell\\Registry.ps1; \
                Write-Host '  Telemetry Service (DiagTrack):' (Get-ServiceStateSafe -ServiceName 'DiagTrack').Status; \
                Write-Host '  Error Reporting (WerSvc):' (Get-ServiceStateSafe -ServiceName 'WerSvc').Status; \
                Write-Host '  Compatibility Appraiser:' (Get-ScheduledTaskStateSafe -TaskPath '\\Microsoft\\Windows\\Application Experience\\' -TaskName 'Microsoft Compatibility Appraiser').State; \
                Write-Host '  AllowTelemetry Policy:' (Get-RegistryValueSafe -Path 'HKLM:\\SOFTWARE\\Policies\\Microsoft\\Windows\\DataCollection' -Name 'AllowTelemetry').Value;"
            ])
            .output()?;

        println!("{}", String::from_utf8_lossy(&output.stdout).trim_end());
        Ok(())
    }

    pub fn apply(dry_run: bool, snapshot: &mut Snapshot) -> Result<()> {
        println!("{}", "[+] Applying Privacy Hardening (Zero Telemetry Leakage)...".cyan());

        let dry_flag = if dry_run { "-WhatIf" } else { "" };
        let cmd = format!(
            ". .\\powershell\\Policies.ps1; . .\\powershell\\Services.ps1; . .\\powershell\\ScheduledTasks.ps1; \
            Set-ObsidianPrivacyPolicies {}; \
            Set-ServiceStateSafe -ServiceName 'DiagTrack' -TargetStartupType 'Disabled' -StopIfRunning {}; \
            Set-ServiceStateSafe -ServiceName 'WerSvc' -TargetStartupType 'Disabled' -StopIfRunning {}; \
            Set-ServiceStateSafe -ServiceName 'MapsBroker' -TargetStartupType 'Disabled' -StopIfRunning {}; \
            Set-ServiceStateSafe -ServiceName 'RetailDemo' -TargetStartupType 'Disabled' -StopIfRunning {}; \
            Set-ScheduledTaskStateSafe -TaskPath '\\Microsoft\\Windows\\Application Experience\\' -TaskName 'Microsoft Compatibility Appraiser' -TargetState 'Disabled' {}; \
            Set-ScheduledTaskStateSafe -TaskPath '\\Microsoft\\Windows\\Application Experience\\' -TaskName 'ProgramDataUpdater' -TargetState 'Disabled' {}; \
            Set-ScheduledTaskStateSafe -TaskPath '\\Microsoft\\Windows\\Customer Experience Improvement Program\\' -TaskName 'Consolidator' -TargetState 'Disabled' {}; \
            Set-ScheduledTaskStateSafe -TaskPath '\\Microsoft\\Windows\\Customer Experience Improvement Program\\' -TaskName 'UsbCeip' -TargetState 'Disabled' {};",
            dry_flag, dry_flag, dry_flag, dry_flag, dry_flag, dry_flag, dry_flag, dry_flag, dry_flag
        );

        let output = Command::new("powershell")
            .args(["-NoProfile", "-ExecutionPolicy", "Bypass", "-Command", &cmd])
            .output()?;

        if output.status.success() {
            println!("{}", "  [OK] Telemetry services, policies, and background tasks configured.".green());
        } else {
            eprintln!("{}", String::from_utf8_lossy(&output.stderr));
        }

        // Record mock snapshot items if not dry-run
        if !dry_run {
            snapshot.services.push(crate::snapshot::SnapshotServiceItem {
                name: "DiagTrack".to_string(),
                previous_startup: "Automatic".to_string(),
                previous_status: "Running".to_string(),
            });
            snapshot.services.push(crate::snapshot::SnapshotServiceItem {
                name: "WerSvc".to_string(),
                previous_startup: "Manual".to_string(),
                previous_status: "Stopped".to_string(),
            });
        }

        Ok(())
    }
}
