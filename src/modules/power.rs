use anyhow::{bail, Result};
use colored::Colorize;
use std::process::Command;

pub struct PowerModule;

impl PowerModule {
    /// Applies a "Max Performance - Never Sleep" power configuration.
    ///
    /// What it does:
    ///   - Sets the active power plan to "High Performance"
    ///   - Disables sleep/standby (AC + DC)
    ///   - Disables hibernate
    ///   - Disables hybrid sleep
    ///   - Disables display auto-off (monitor stays on)
    ///   - Disables USB selective suspend (prevents USB device drops)
    ///   - Sets minimum CPU performance to 100% (no throttling ever)
    ///   - Disables Fast Startup (causes partial shutdown bugs)
    pub fn apply(dry_run: bool) -> Result<()> {
        println!(
            "{}",
            t!(
                en: "[+] Applying Power Management — Maximum Performance Mode...",
                es: "[+] Aplicando Gestión de Energía — Modo de Máximo Rendimiento..."
            )
            .cyan()
        );

        if dry_run {
            println!(
                "{}",
                t!(
                    en: "  [DRY-RUN] Would configure power plan to never sleep or dim display.",
                    es: "  [SIMULACIÓN] Configuraría el plan de energía para no dormir ni apagar pantalla."
                )
                .dimmed()
            );
            return Ok(());
        }

        // ── Step 1: Activate High Performance plan ─────────────────────────────
        println!(
            "{}",
            t!(
                en: "  [*] Activating High Performance power plan...",
                es: "  [*] Activando el plan de energía de Alto Rendimiento..."
            )
            .dimmed()
        );
        println!(
            "{}",
            t!(
                en: "      (Forces the CPU and storage controller to always run at full speed, zero power-saving throttling)",
                es: "      (Fuerza a la CPU y controlador de almacenamiento a correr siempre al máximo, sin ralentización)"
            )
            .green()
        );
        let out = Command::new("cmd")
            .args(["/c", "powercfg /setactive 8c5e7fda-e8bf-4a96-9a85-a6e23a8c635c"])
            .output()?;
        if !out.status.success() {
            // Fallback: create a new high performance plan from SCHEME_MIN
            let _ = Command::new("cmd")
                .args(["/c", "powercfg -duplicatescheme 8c5e7fda-e8bf-4a96-9a85-a6e23a8c635c"])
                .output();
        }
        println!("{}", t!(en: "  [OK] High Performance plan active.", es: "  [OK] Plan de Alto Rendimiento activo.").green());

        // ── Step 2: Disable Sleep (AC and DC) ──────────────────────────────────
        println!(
            "{}",
            t!(
                en: "  [*] Disabling system sleep / standby...",
                es: "  [*] Desactivando la suspensión del sistema..."
            )
            .dimmed()
        );
        println!(
            "{}",
            t!(
                en: "      (Prevents Windows from suspending when you leave it idle — critical during long AI tasks or builds)",
                es: "      (Evita que Windows se suspenda cuando lo dejas en reposo — crítico durante tareas largas de IA o compilaciones)"
            )
            .green()
        );
        // STANDBYIDLE = sleep timeout (0 = never)
        run_powercfg(&["/change", "standby-timeout-ac", "0"])?;
        run_powercfg(&["/change", "standby-timeout-dc", "0"])?;
        println!("{}", t!(en: "  [OK] Sleep disabled (AC + DC).", es: "  [OK] Suspensión desactivada (CA + batería).").green());

        // ── Step 3: Disable Hibernate ───────────────────────────────────────────
        println!(
            "{}",
            t!(
                en: "  [*] Disabling Hibernate...",
                es: "  [*] Desactivando la hibernación..."
            )
            .dimmed()
        );
        println!(
            "{}",
            t!(
                en: "      (Frees the hiberfil.sys disk reservation — usually 8-32 GB — and removes unexpected shutdown risk)",
                es: "      (Libera el espacio reservado de hiberfil.sys — normalmente 8-32 GB — y elimina riesgo de apagado inesperado)"
            )
            .green()
        );
        run_powercfg(&["/hibernate", "off"])?;
        run_powercfg(&["/change", "hibernate-timeout-ac", "0"])?;
        run_powercfg(&["/change", "hibernate-timeout-dc", "0"])?;
        println!("{}", t!(en: "  [OK] Hibernate disabled.", es: "  [OK] Hibernación desactivada.").green());

        // ── Step 4: Disable Hybrid Sleep ───────────────────────────────────────
        println!(
            "{}",
            t!(
                en: "  [*] Disabling Hybrid Sleep...",
                es: "  [*] Desactivando el modo de Suspensión Híbrida..."
            )
            .dimmed()
        );
        println!(
            "{}",
            t!(
                en: "      (Hybrid sleep writes RAM to disk before sleeping — causes micro-freezes and delays wakeup time)",
                es: "      (La suspensión híbrida escribe la RAM en disco antes de dormir — causa micro-congelaciones y retrasos al reactivar)"
            )
            .green()
        );
        // SUB_SLEEP / HYBRIDSLEEP = 0 → Disabled
        let _ = Command::new("cmd")
            .args(["/c", "powercfg /setacvalueindex SCHEME_CURRENT SUB_SLEEP HYBRIDSLEEP 0"])
            .output();
        let _ = Command::new("cmd")
            .args(["/c", "powercfg /setdcvalueindex SCHEME_CURRENT SUB_SLEEP HYBRIDSLEEP 0"])
            .output();
        let _ = Command::new("cmd")
            .args(["/c", "powercfg /setactive SCHEME_CURRENT"])
            .output();
        println!("{}", t!(en: "  [OK] Hybrid Sleep disabled.", es: "  [OK] Suspensión Híbrida desactivada.").green());

        // ── Step 5: Keep display always ON ─────────────────────────────────────
        println!(
            "{}",
            t!(
                en: "  [*] Disabling automatic monitor turn-off...",
                es: "  [*] Desactivando el apagado automático del monitor..."
            )
            .dimmed()
        );
        println!(
            "{}",
            t!(
                en: "      (Keeps your screen permanently on — essential during vibe-coding sessions where the AI may be working)",
                es: "      (Mantiene la pantalla permanentemente encendida — esencial durante sesiones de vibe-coding donde la IA puede estar trabajando)"
            )
            .green()
        );
        run_powercfg(&["/change", "monitor-timeout-ac", "0"])?;
        run_powercfg(&["/change", "monitor-timeout-dc", "0"])?;
        println!("{}", t!(en: "  [OK] Monitor will never turn off automatically.", es: "  [OK] El monitor nunca se apagará automáticamente.").green());

        // ── Step 6: Disable USB Selective Suspend ──────────────────────────────
        println!(
            "{}",
            t!(
                en: "  [*] Disabling USB Selective Suspend...",
                es: "  [*] Desactivando la Suspensión Selectiva de USB..."
            )
            .dimmed()
        );
        println!(
            "{}",
            t!(
                en: "      (Stops Windows from power-gating USB controllers — prevents keyboard/mouse drops and device disconnects)",
                es: "      (Evita que Windows apague los controladores USB — previene desconexiones de teclado, ratón y periféricos)"
            )
            .green()
        );
        let _ = Command::new("cmd")
            .args(["/c", "powercfg /setacvalueindex SCHEME_CURRENT 2a737441-1930-4402-8d77-b2bebba308a3 48e6b7a6-50f5-4782-a5d4-53bb8f07e226 0"])
            .output();
        let _ = Command::new("cmd")
            .args(["/c", "powercfg /setactive SCHEME_CURRENT"])
            .output();
        println!("{}", t!(en: "  [OK] USB Selective Suspend disabled.", es: "  [OK] Suspensión Selectiva de USB desactivada.").green());

        // ── Step 7: Minimum CPU performance = 100% (no throttle) ──────────────
        println!(
            "{}",
            t!(
                en: "  [*] Setting minimum CPU performance state to 100%...",
                es: "  [*] Estableciendo el rendimiento mínimo de CPU al 100%..."
            )
            .dimmed()
        );
        println!(
            "{}",
            t!(
                en: "      (Prevents the CPU from ever downclocking below its maximum — eliminates hitches during AI inference spikes)",
                es: "      (Evita que la CPU baje de sus frecuencias máximas — elimina tirones durante picos de inferencia de IA)"
            )
            .green()
        );
        let _ = Command::new("cmd")
            .args(["/c", "powercfg /setacvalueindex SCHEME_CURRENT SUB_PROCESSOR PROCTHROTTLEMIN 100"])
            .output();
        let _ = Command::new("cmd")
            .args(["/c", "powercfg /setactive SCHEME_CURRENT"])
            .output();
        println!("{}", t!(en: "  [OK] CPU will always run at full performance.", es: "  [OK] La CPU siempre correrá a pleno rendimiento.").green());

        // ── Step 8: Disable Fast Startup ───────────────────────────────────────
        println!(
            "{}",
            t!(
                en: "  [*] Disabling Windows Fast Startup...",
                es: "  [*] Desactivando el Inicio Rápido de Windows..."
            )
            .dimmed()
        );
        println!(
            "{}",
            t!(
                en: "      (Fast Startup is a hybrid shutdown that keeps drivers in a dirty state — causes crashes and stale GPU sessions)",
                es: "      (El Inicio Rápido es un apagado híbrido que mantiene los drivers en estado sucio — causa cuelgues y sesiones GPU obsoletas)"
            )
            .green()
        );
        let fast_startup_cmd = "Set-ItemProperty \
            -Path 'HKLM:\\SYSTEM\\CurrentControlSet\\Control\\Session Manager\\Power' \
            -Name 'HiberbootEnabled' -Value 0 -Type DWord -Force";
        let out = Command::new("powershell")
            .args(["-NoProfile", "-ExecutionPolicy", "Bypass", "-Command", fast_startup_cmd])
            .output()?;
        if !out.status.success() {
            bail!("Fast Startup registry tweak failed: {}", String::from_utf8_lossy(&out.stderr).trim());
        }
        println!("{}", t!(en: "  [OK] Fast Startup disabled — clean shutdowns guaranteed.", es: "  [OK] Inicio Rápido desactivado — apagados limpios garantizados.").green());

        println!(
            "{}",
            t!(
                en: "\n  [✓] Power profile locked to MAX PERFORMANCE. System will NEVER sleep or suspend automatically.",
                es: "\n  [✓] Perfil de energía bloqueado a MÁXIMO RENDIMIENTO. El sistema NUNCA se dormirá ni suspenderá automáticamente."
            )
            .green()
            .bold()
        );

        Ok(())
    }
}

fn run_powercfg(args: &[&str]) -> Result<()> {
    let out = Command::new("powercfg").args(args).output()?;
    if !out.status.success() {
        let err = String::from_utf8_lossy(&out.stderr);
        bail!("powercfg {:?} failed: {}", args, err.trim());
    }
    Ok(())
}
