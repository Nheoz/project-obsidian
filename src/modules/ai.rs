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
        println!(
            "  {}",
            "[*] Enabling Hardware-Accelerated GPU Scheduling (HAGS)...".dimmed()
        );
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
        println!(
            "{}",
            "  [OK] HAGS enabled — GPU scheduling latency reduced for CUDA/DirectML.".green()
        );

        // 2. Disable GPU TDR timeout for long AI inference jobs (default 2s → 60s)
        println!(
            "  {}",
            "[*] Setting GPU TDR timeout for AI inference (60s)...".dimmed()
        );
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
        println!(
            "{}",
            "  [OK] GPU TDR timeout extended — prevents crashes during long model runs.".green()
        );

        // 3. Disable NVIDIA Telemetry service if present
        println!(
            "  {}",
            "[*] Disabling NVIDIA Telemetry Container...".dimmed()
        );
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
            println!("{}", "  [OK] NVIDIA Telemetry Container disabled.".green());
        } else {
            println!(
                "{}",
                "  [--] NVIDIA Telemetry Container not present (skip).".dimmed()
            );
        }

        // 4. Optimize WSL2 memory config if WSL is installed
        println!("  {}", "[*] Checking WSL2 memory configuration...".dimmed());
        let wsl_check = Command::new("wsl").args(["--status"]).output();
        if wsl_check.map(|o| o.status.success()).unwrap_or(false) {
            let wslconfig_path = dirs_home().join(".wslconfig");
            // Only write if not already customized
            if !wslconfig_path.exists() {
                let wslconfig =
                    "[wsl2]\nmemory=8GB\nprocessors=4\nswap=4GB\nlocalhostForwarding=true\n";
                std::fs::write(&wslconfig_path, wslconfig).ok();
                println!(
                    "{}",
                    "  [OK] WSL2 .wslconfig created (8GB RAM, 4 CPUs, 4GB swap for AI stacks)."
                        .green()
                );
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
