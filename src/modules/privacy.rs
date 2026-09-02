use crate::snapshot::{Snapshot, SnapshotServiceItem, SnapshotTaskItem};
use anyhow::{bail, Result};
use colored::*;
use std::process::Command;

pub struct PrivacyModule;

/// Services that Privacy hardening manages, with expected baseline for fresh Windows 11
const PRIVACY_SERVICES: &[&str] = &["DiagTrack", "WerSvc", "MapsBroker", "RetailDemo"];

/// Scheduled tasks that Privacy hardening manages
const PRIVACY_TASKS: &[(&str, &str)] = &[
    (
        "\\Microsoft\\Windows\\Application Experience\\",
        "Microsoft Compatibility Appraiser",
    ),
    (
        "\\Microsoft\\Windows\\Application Experience\\",
        "ProgramDataUpdater",
    ),
    (
        "\\Microsoft\\Windows\\Customer Experience Improvement Program\\",
        "Consolidator",
    ),
    (
        "\\Microsoft\\Windows\\Customer Experience Improvement Program\\",
        "UsbCeip",
    ),
];

impl PrivacyModule {
    pub fn audit() -> Result<()> {
        println!("{}", "--- [Privacy Posture Audit] ---".yellow());
        let root = crate::embedded::get_scripts_root();
        let svc_p = root.join("powershell").join("Services.ps1");
        let tsk_p = root.join("powershell").join("ScheduledTasks.ps1");
        let reg_p = root.join("powershell").join("Registry.ps1");

        let cmd = format!(
            ". '{}'; . '{}'; . '{}'; \
            Write-Host '  Telemetry Service (DiagTrack):' (Get-ServiceStateSafe -ServiceName 'DiagTrack').Status; \
            Write-Host '  Error Reporting (WerSvc):' (Get-ServiceStateSafe -ServiceName 'WerSvc').Status; \
            Write-Host '  Compatibility Appraiser:' (Get-ScheduledTaskStateSafe -TaskPath '\\Microsoft\\Windows\\Application Experience\\' -TaskName 'Microsoft Compatibility Appraiser').State; \
            Write-Host '  AllowTelemetry Policy:' (Get-RegistryValueSafe -Path 'HKLM:\\SOFTWARE\\Policies\\Microsoft\\Windows\\DataCollection' -Name 'AllowTelemetry').Value;",
            svc_p.display(), tsk_p.display(), reg_p.display()
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
            "[+] Applying Privacy Hardening (Zero Telemetry Leakage)...".cyan()
        );

        let root = crate::embedded::get_scripts_root();
        let pol_p = root.join("powershell").join("Policies.ps1");
        let svc_p = root.join("powershell").join("Services.ps1");
        let tsk_p = root.join("powershell").join("ScheduledTasks.ps1");

        // ── PHASE 0: Capture real current state BEFORE making any changes ──────────
        if !dry_run {
            println!(
                "{}",
                "  [*] Capturing real system state for rollback...".dimmed()
            );
            snapshot
                .services
                .extend(Self::capture_service_states(&svc_p.to_string_lossy())?);
            snapshot
                .tasks
                .extend(Self::capture_task_states(&tsk_p.to_string_lossy())?);
        }

        // ── PHASE 1: Apply changes ───────────────────────────────────────────────
        let dry_flag = if dry_run { "-WhatIf" } else { "" };
        let cmd = format!(
            ". '{}'; . '{}'; . '{}'; \
            Set-ObsidianPrivacyPolicies {}; \
            Set-ServiceStateSafe -ServiceName 'DiagTrack' -TargetStartupType 'Disabled' -StopIfRunning {}; \
            Set-ServiceStateSafe -ServiceName 'WerSvc' -TargetStartupType 'Disabled' -StopIfRunning {}; \
            Set-ServiceStateSafe -ServiceName 'MapsBroker' -TargetStartupType 'Disabled' -StopIfRunning {}; \
            Set-ServiceStateSafe -ServiceName 'RetailDemo' -TargetStartupType 'Disabled' -StopIfRunning {}; \
            Set-ScheduledTaskStateSafe -TaskPath '\\Microsoft\\Windows\\Application Experience\\' -TaskName 'Microsoft Compatibility Appraiser' -TargetState 'Disabled' {}; \
            Set-ScheduledTaskStateSafe -TaskPath '\\Microsoft\\Windows\\Application Experience\\' -TaskName 'ProgramDataUpdater' -TargetState 'Disabled' {}; \
            Set-ScheduledTaskStateSafe -TaskPath '\\Microsoft\\Windows\\Customer Experience Improvement Program\\' -TaskName 'Consolidator' -TargetState 'Disabled' {}; \
            Set-ScheduledTaskStateSafe -TaskPath '\\Microsoft\\Windows\\Customer Experience Improvement Program\\' -TaskName 'UsbCeip' -TargetState 'Disabled' {};",
            pol_p.display(), svc_p.display(), tsk_p.display(),
            dry_flag, dry_flag, dry_flag, dry_flag, dry_flag, dry_flag, dry_flag, dry_flag, dry_flag
        );

        println!("{}", t!(
            en: "  [*] Disabling Telemetry & Error Reporting services...",
            es: "  [*] Desactivando servicios de Telemetría y Reporte de Errores..."
        ).dimmed());
        println!("{}", t!(
            en: "      (Stops Windows from constantly sending diagnostic data and crash dumps to Microsoft servers)",
            es: "      (Evita que Windows envíe datos de diagnóstico y volcados de errores a Microsoft)"
        ).green());

        println!("{}", t!(
            en: "  [*] Disabling diagnostic background tasks...",
            es: "  [*] Desactivando tareas de diagnóstico en segundo plano..."
        ).dimmed());
        println!("{}", t!(
            en: "      (Prevents scheduled telemetry sweeps like Compatibility Appraiser from burning CPU cycles)",
            es: "      (Evita barridos de telemetría como el Compatibility Appraiser que consumen CPU inútilmente)"
        ).green());

        println!("{}", t!(
            en: "  [*] Enforcing Local Privacy Group Policies...",
            es: "  [*] Aplicando Políticas de Grupo de Privacidad..."
        ).dimmed());
        println!("{}", t!(
            en: "      (Blocks advertising ID tracking, activity feed history, and typing data collection)",
            es: "      (Bloquea el ID de publicidad, el historial de actividad y la recolección de datos de escritura)"
        ).green());

        let output = Command::new("powershell")
            .args(["-NoProfile", "-ExecutionPolicy", "Bypass", "-Command", &cmd])
            .output()?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            bail!(
                "Privacy hardening PowerShell script failed:\n{}",
                stderr.trim()
            );
        }

        println!(
            "{}",
            "  [OK] Telemetry services, policies, and background tasks secured.".green()
        );
        Ok(())
    }

    /// Read the real startup type and status of each privacy service from the live system.
    fn capture_service_states(svc_script: &str) -> Result<Vec<SnapshotServiceItem>> {
        let names: Vec<String> = PRIVACY_SERVICES
            .iter()
            .map(|s| format!("'{}'", s))
            .collect();
        let list = names.join(",");

        let cmd = format!(
            ". '{}'; @({}) | ForEach-Object {{ Get-ServiceStateSafe -ServiceName $_ }} | ConvertTo-Json -Compress",
            svc_script, list
        );

        let output = Command::new("powershell")
            .args(["-NoProfile", "-ExecutionPolicy", "Bypass", "-Command", &cmd])
            .output()?;

        if !output.status.success() {
            bail!(
                "Failed to read service states for snapshot: {}",
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
            // Single object returned when only one service
            vec![val]
        };

        for item in arr {
            let name = item["Name"].as_str().unwrap_or("").to_string();
            if name.is_empty() {
                continue;
            }
            items.push(SnapshotServiceItem {
                name,
                previous_startup: item["StartupType"]
                    .as_str()
                    .unwrap_or("Unknown")
                    .to_string(),
                previous_status: item["Status"].as_str().unwrap_or("Unknown").to_string(),
            });
        }

        Ok(items)
    }

    /// Read the real state of each privacy scheduled task from the live system.
    fn capture_task_states(tsk_script: &str) -> Result<Vec<SnapshotTaskItem>> {
        let mut parts = Vec::new();
        for (path, name) in PRIVACY_TASKS {
            parts.push(format!(
                "Get-ScheduledTaskStateSafe -TaskPath '{}' -TaskName '{}'",
                path, name
            ));
        }
        let queries = parts.join("; ");

        let cmd = format!(
            ". '{}'; @({}) | ConvertTo-Json -Compress",
            tsk_script, queries
        );

        let output = Command::new("powershell")
            .args(["-NoProfile", "-ExecutionPolicy", "Bypass", "-Command", &cmd])
            .output()?;

        if !output.status.success() {
            bail!(
                "Failed to read scheduled task states for snapshot: {}",
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
            let path = item["TaskPath"].as_str().unwrap_or("").to_string();
            let name = item["TaskName"].as_str().unwrap_or("").to_string();
            if name.is_empty() {
                continue;
            }
            items.push(SnapshotTaskItem {
                path,
                name,
                previous_state: item["State"].as_str().unwrap_or("Unknown").to_string(),
            });
        }

        Ok(items)
    }
}
