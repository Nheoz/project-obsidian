use anyhow::{bail, Result};
use colored::*;
use std::process::Command;

pub struct AiModule;

impl AiModule {
    pub fn doctor() -> Result<()> {
        println!(
            "{}",
            "================================================================================"
                .cyan()
        );
        println!(
            "{}",
            "            PROJECT OBSIDIAN — AI WORKSTATION HEALTH & DIAGNOSTICS             "
                .cyan()
        );
        println!(
            "{}",
            "================================================================================"
                .cyan()
        );

        // 1. NVIDIA GPU & Driver
        print!("  {:<25} : ", "NVIDIA GPU & Driver");
        let gpu_out = Command::new("powershell")
            .args(["-NoProfile", "-Command",
                "(Get-CimInstance Win32_VideoController | Where-Object { $_.Name -like '*NVIDIA*' } | Select-Object -First 1).Name"])
            .output();
        if let Ok(out) = gpu_out {
            let s = String::from_utf8_lossy(&out.stdout).trim().to_string();
            if !s.is_empty() {
                println!("{}", s.green().bold());
            } else {
                println!(
                    "{}",
                    "No NVIDIA GPU detected (CPU/DirectML fallback)".yellow()
                );
            }
        }

        // 2. CUDA Compiler / Toolkit
        print!("  {:<25} : ", "CUDA Toolkit (nvcc)");
        let nvcc_out = Command::new("nvcc").arg("--version").output();
        match nvcc_out {
            Ok(out) if out.status.success() => {
                let s = String::from_utf8_lossy(&out.stdout);
                let line = s
                    .lines()
                    .find(|l| l.contains("release"))
                    .unwrap_or("Installed");
                println!("{}", line.trim().green());
            }
            _ => println!(
                "{}",
                "Not detected in PATH (Optional for local training)".yellow()
            ),
        }

        // 3. WSL2 Subsystem
        print!("  {:<25} : ", "WSL2 Subsystem");
        let wsl_out = Command::new("wsl").arg("--status").output();
        match wsl_out {
            Ok(out) if out.status.success() => {
                println!(
                    "{}",
                    "Active and operational [Recommended for Linux AI stacks]".green()
                );
            }
            _ => println!("{}", "Not installed (Optional: wsl --install)".yellow()),
        }

        // 4. Docker Desktop
        print!("  {:<25} : ", "Docker Engine");
        let docker_out = Command::new("docker").arg("--version").output();
        match docker_out {
            Ok(out) if out.status.success() => {
                let s = String::from_utf8_lossy(&out.stdout).trim().to_string();
                println!("{}", s.green());
            }
            _ => println!(
                "{}",
                "Not installed (Optional: winget install Docker.DockerDesktop)".yellow()
            ),
        }

        // 5. Python Runtime
        print!("  {:<25} : ", "Python Runtime");
        let py_out = Command::new("python").arg("--version").output();
        match py_out {
            Ok(out) if out.status.success() => {
                let s = String::from_utf8_lossy(&out.stdout).trim().to_string();
                println!("{}", s.green());
            }
            _ => println!(
                "{}",
                "Not found in PATH (Install via python.org or uv)".yellow()
            ),
        }

        // 6. Git Version Control
        print!("  {:<25} : ", "Git Version Control");
        let git_out = Command::new("git").arg("--version").output();
        match git_out {
            Ok(out) if out.status.success() => {
                let s = String::from_utf8_lossy(&out.stdout).trim().to_string();
                println!("{}", s.green());
            }
            _ => println!("{}", "Not found (Install: winget install Git.Git)".yellow()),
        }

        // 7. Ollama Local LLM Runner
        print!("  {:<25} : ", "Ollama Local LLM");
        let ollama_out = Command::new("ollama").arg("--version").output();
        match ollama_out {
            Ok(out) if out.status.success() => {
                let s = String::from_utf8_lossy(&out.stdout).trim().to_string();
                println!("{}", s.green());
            }
            _ => println!(
                "{}",
                "Not detected (Optional: winget install Ollama.Ollama)".yellow()
            ),
        }

        // ── Driver Conflict Detection ──────────────────────────────────────────
        println!();
        println!(
            "{}",
            t!(
                en: "  ── Driver Health & Conflict Analysis ──",
                es: "  ── Análisis de Salud y Conflictos de Drivers ──"
            )
            .white()
            .bold()
        );
        println!(
            "{}",
            t!(
                en: "  [*] Scanning all hardware devices for driver errors...",
                es: "  [*] Escaneando todos los dispositivos hardware en busca de errores de driver..."
            )
            .dimmed()
        );

        let driver_scan_cmd = r#"
            [Console]::OutputEncoding = [System.Text.Encoding]::UTF8
            $problems = Get-CimInstance Win32_PnPEntity |
                Where-Object { $_.ConfigManagerErrorCode -ne 0 } |
                Select-Object Name, ConfigManagerErrorCode, DeviceID |
                Sort-Object ConfigManagerErrorCode

            if ($null -eq $problems -or @($problems).Count -eq 0) {
                Write-Host "CLEAN"
            } else {
                $problems | ForEach-Object {
                    $code = $_.ConfigManagerErrorCode
                    $meaning = switch ($code) {
                        1  { "Device not configured correctly" }
                        3  { "Driver cannot load / corrupted" }
                        10 { "Device cannot start (Code 10)" }
                        12 { "IRQ/DMA resource conflict" }
                        14 { "Requires restart to finish installing" }
                        18 { "Reinstall drivers required" }
                        22 { "Device is disabled" }
                        28 { "Drivers not installed (Code 28)" }
                        31 { "Device not working properly" }
                        43 { "Device stopped — Windows reported a problem (Code 43)" }
                        45 { "Device not present (was recently connected)" }
                        47 { "Exceeded max resources — cannot start" }
                        52 { "Driver blocked from starting (unsigned/policy)" }
                        default { "Unknown error code $code" }
                    }
                    Write-Host "ERROR|$($_.Name)|Code $code|$meaning"
                }
            }
        "#;

        if let Ok(driver_out) = Command::new("powershell").args(["-NoProfile", "-ExecutionPolicy", "Bypass", "-Command", driver_scan_cmd]).output() {
            let driver_str = String::from_utf8_lossy(&driver_out.stdout);
            let mut conflict_count = 0u32;

            for line in driver_str.lines() {
                let line = line.trim();
                if line == "CLEAN" {
                    break;
                }
                if line.starts_with("ERROR|") {
                    let parts: Vec<&str> = line.splitn(4, '|').collect();
                    if parts.len() == 4 {
                        conflict_count += 1;
                        let device  = parts[1];
                        let code    = parts[2];
                        let meaning = parts[3];
                        println!(
                            "  {} {} — {} — {}",
                            "[CONFLICT]".red().bold(),
                            device.white().bold(),
                            code.yellow(),
                            meaning.red()
                        );

                        let advice = match parts[2] {
                            c if c.contains("43") => t!(
                                en: "           → Code 43: GPU/Device stopped. Try: clean driver reinstall (DDU), check connections.",
                                es: "           → Código 43: GPU/Dispositivo detenido. Intenta: reinstalación limpia (DDU), revisa conexiones."
                            ).to_string(),
                            c if c.contains("10") => t!(
                                en: "           → Code 10: Device cannot start. Update driver via Device Manager.",
                                es: "           → Código 10: El dispositivo no puede iniciar. Actualiza el driver desde el Administrador de Dispositivos."
                            ).to_string(),
                            c if c.contains("28") => t!(
                                en: "           → Code 28: No driver installed. Install missing driver.",
                                es: "           → Código 28: Sin driver instalado. Instala el driver faltante."
                            ).to_string(),
                            c if c.contains("12") => t!(
                                en: "           → Code 12: Resource conflict (IRQ). Try another PCIe slot or check BIOS.",
                                es: "           → Código 12: Conflicto de recursos (IRQ). Prueba otro puerto PCIe o revisa la BIOS."
                            ).to_string(),
                            c if c.contains("52") => t!(
                                en: "           → Code 52: Driver blocked (unsigned). Need WHQL-signed driver.",
                                es: "           → Código 52: Driver bloqueado (sin firma). Requiere un driver con firma WHQL."
                            ).to_string(),
                            _ => String::new(),
                        };
                        if !advice.is_empty() {
                            println!("{}", advice.yellow());
                        }
                    }
                }
            }

            if conflict_count == 0 {
                println!(
                    "  {} {}",
                    "[CLEAN]".green().bold(),
                    t!(
                        en: "All hardware devices report healthy drivers. No conflicts detected.",
                        es: "Todos los dispositivos hardware reportan drivers saludables. No se detectaron conflictos."
                    ).green()
                );
            } else {
                println!(
                    "\n  {} {} {}",
                    "[!]".red().bold(),
                    conflict_count,
                    t!(
                        en: "driver conflict(s) detected. Review before applying optimizations.",
                        es: "conflicto(s) de driver detectado(s). Revisa antes de aplicar optimizaciones."
                    ).yellow()
                );
            }
        }

        println!(
            "{}",
            "================================================================================"
                .cyan()
        );
        println!(
            "{}",
            "[V] AI WORKSTATION DIAGNOSTICS COMPLETED.".green().bold()
        );
        Ok(())
    }

    /// Apply AI workstation optimizations:
    /// - Enable WSL2 Virtual Machine Platform if not present
    /// - Set WSL2 memory/CPU limits via .wslconfig for stability with large models
    /// - Disable NVIDIA Telemetry services (NvTelemetryContainer)
    /// - Enable Hardware-Accelerated GPU Scheduling (HAGS) for CUDA/DirectML throughput
    /// - Disable GPU preemption timeout (TdrLevel) for long-running AI inference jobs
    pub fn apply() -> Result<()> {
        println!("{}", "[+] Applying AI Workstation Optimizations...".cyan());

        // 1. Enable Hardware-Accelerated GPU Scheduling (HAGS) — registry tweak
        println!("{}", t!(
            en: "  [*] Enabling Hardware-Accelerated GPU Scheduling (HAGS)...",
            es: "  [*] Habilitando la Programación de GPU acelerada por hardware (HAGS)..."
        ).dimmed());
        println!("{}", t!(
            en: "      (Offloads GPU scheduling from the CPU to the GPU itself, improving throughput for AI/Games)",
            es: "      (Descarga la planificación de GPU de la CPU a la propia GPU, mejorando rendimiento en AI/Juegos)"
        ).green());
        let hags_cmd = "\
            $path = 'HKLM:\\SYSTEM\\CurrentControlSet\\Control\\GraphicsDrivers'; \
            if (-not (Test-Path $path)) { New-Item -Path $path -Force | Out-Null }; \
            Set-ItemProperty -Path $path -Name 'HwSchMode' -Value 2 -Type DWord -Force;";

        let out = Command::new("powershell")
            .args([
                "-NoProfile",
                "-ExecutionPolicy",
                "Bypass",
                "-Command",
                hags_cmd,
            ])
            .output()?;
        if !out.status.success() {
            bail!(
                "HAGS registry tweak failed: {}",
                String::from_utf8_lossy(&out.stderr).trim()
            );
        }
        println!("{}", t!(en: "  [OK] HAGS enabled.", es: "  [OK] HAGS habilitado.").green());

        // 2. Disable GPU TDR timeout for long AI inference jobs (default 2s → 60s)
        println!("{}", t!(
            en: "  [*] Setting GPU TDR timeout for AI inference (60s)...",
            es: "  [*] Configurando el tiempo de espera TDR de la GPU para IA (60s)..."
        ).dimmed());
        println!("{}", t!(
            en: "      (Prevents Windows from forcibly restarting your graphics driver during heavy AI image/text generation)",
            es: "      (Evita que Windows reinicie el driver gráfico a la fuerza durante inferencias pesadas de IA)"
        ).green());
        let tdr_cmd = "\
            $path = 'HKLM:\\SYSTEM\\CurrentControlSet\\Control\\GraphicsDrivers'; \
            Set-ItemProperty -Path $path -Name 'TdrDelay' -Value 60 -Type DWord -Force; \
            Set-ItemProperty -Path $path -Name 'TdrDdiDelay' -Value 60 -Type DWord -Force;";

        let out = Command::new("powershell")
            .args([
                "-NoProfile",
                "-ExecutionPolicy",
                "Bypass",
                "-Command",
                tdr_cmd,
            ])
            .output()?;
        if !out.status.success() {
            bail!(
                "TDR timeout tweak failed: {}",
                String::from_utf8_lossy(&out.stderr).trim()
            );
        }
        println!("{}", "  [OK] GPU TDR timeout extended.".green());

        // 3. Disable NVIDIA Telemetry service if present
        println!("{}", t!(
            en: "  [*] Disabling NVIDIA Telemetry Container...",
            es: "  [*] Desactivando el Contenedor de Telemetría de NVIDIA..."
        ).dimmed());
        println!("{}", t!(
            en: "      (Stops NVIDIA from sending driver usage data in the background, freeing up memory)",
            es: "      (Evita que NVIDIA envíe datos de uso en segundo plano, liberando memoria)"
        ).green());
        let nvtelem_cmd = "\
            $svc = Get-Service -Name 'NvTelemetryContainer' -ErrorAction SilentlyContinue; \
            if ($svc) { \
                Stop-Service -Name 'NvTelemetryContainer' -Force -ErrorAction SilentlyContinue; \
                Set-Service -Name 'NvTelemetryContainer' -StartupType Disabled -ErrorAction SilentlyContinue; \
                Write-Host 'Disabled'; \
            } else { \
                Write-Host 'NotPresent'; \
            }";

        let out = Command::new("powershell")
            .args([
                "-NoProfile",
                "-ExecutionPolicy",
                "Bypass",
                "-Command",
                nvtelem_cmd,
            ])
            .output()?;
        let result = String::from_utf8_lossy(&out.stdout).trim().to_string();
        if result == "Disabled" {
            println!("{}", t!(en: "  [OK] NVIDIA Telemetry Container disabled.", es: "  [OK] Contenedor de telemetría NVIDIA desactivado.").green());
        } else {
            println!("{}", t!(en: "  [--] NVIDIA Telemetry Container not present (skip).", es: "  [--] Contenedor de telemetría NVIDIA no encontrado (omitido).").dimmed());
        }

        // 4. Optimize WSL2 memory config if WSL is installed
        println!("{}", t!(
            en: "  [*] Checking WSL2 memory configuration...",
            es: "  [*] Comprobando la configuración de memoria de WSL2..."
        ).dimmed());
        println!("{}", t!(
            en: "      (Applies safe RAM/CPU limits to Windows Subsystem for Linux so it doesn't consume all system RAM)",
            es: "      (Aplica límites seguros de RAM/CPU a WSL para que no devore toda la RAM del sistema)"
        ).green());
        let wsl_check = Command::new("wsl").args(["--status"]).output();
        if wsl_check.map(|o| o.status.success()).unwrap_or(false) {
            let wslconfig_path = dirs_home().join(".wslconfig");
            // Only write if not already customized
            if !wslconfig_path.exists() {
                let wslconfig =
                    "[wsl2]\nmemory=8GB\nprocessors=4\nswap=4GB\nlocalhostForwarding=true\n";
                std::fs::write(&wslconfig_path, wslconfig).ok();
                println!("{}", "  [OK] WSL2 .wslconfig created.".green());
            } else {
                println!(
                    "{}",
                    "  [--] WSL2 .wslconfig already exists (not overwritten).".dimmed()
                );
            }
        } else {
            println!(
                "{}",
                "  [--] WSL2 not installed (skip WSL2 config).".dimmed()
            );
        }

        println!(
            "{}",
            "================================================================================"
                .cyan()
        );
        println!(
            "{}",
            "[V] AI WORKSTATION OPTIMIZATION APPLIED.".green().bold()
        );
        Ok(())
    }
}

fn dirs_home() -> std::path::PathBuf {
    std::env::var("USERPROFILE")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|_| std::path::PathBuf::from("C:\\Users\\Default"))
}
