use anyhow::Result;
use colored::*;
use std::process::Command;

pub struct AiModule;

impl AiModule {
    pub fn doctor() -> Result<()> {
        println!("{}", "================================================================================".cyan());
        println!("{}", "            PROJECT OBSIDIAN — AI WORKSTATION HEALTH & DIAGNOSTICS             ".cyan());
        println!("{}", "================================================================================".cyan());

        // 1. NVIDIA GPU & Driver
        print!("  {:<25} : ", "NVIDIA GPU & Driver");
        let gpu_out = Command::new("powershell")
            .args(["-NoProfile", "-Command", "(Get-CimInstance Win32_VideoController | Where-Object { $_.Name -like '*NVIDIA*' } | Select-Object -First 1).Name"])
            .output();
        if let Ok(out) = gpu_out {
            let s = String::from_utf8_lossy(&out.stdout).trim().to_string();
            if !s.is_empty() {
                println!("{}", s.green().bold());
            } else {
                println!("{}", "No NVIDIA GPU detected (CPU/DirectML fallback)".yellow());
            }
        }

        // 2. CUDA Compiler / Toolkit
        print!("  {:<25} : ", "CUDA Toolkit (nvcc)");
        let nvcc_out = Command::new("nvcc").arg("--version").output();
        match nvcc_out {
            Ok(out) if out.status.success() => {
                let s = String::from_utf8_lossy(&out.stdout);
                let line = s.lines().find(|l| l.contains("release")).unwrap_or("Installed");
                println!("{}", line.trim().green());
            }
            _ => println!("{}", "Not detected in PATH (Optional for local training)".yellow()),
        }

        // 3. WSL2 Subsystem
        print!("  {:<25} : ", "WSL2 Subsystem");
        let wsl_out = Command::new("wsl").arg("--status").output();
        match wsl_out {
            Ok(out) if out.status.success() => {
                println!("{}", "Active and operational [Recommended for Linux AI stacks]".green());
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
            _ => println!("{}", "Not installed (Optional: winget install Docker.DockerDesktop)".yellow()),
        }

        // 5. Python Runtime
        print!("  {:<25} : ", "Python Runtime");
        let py_out = Command::new("python").arg("--version").output();
        match py_out {
            Ok(out) if out.status.success() => {
                let s = String::from_utf8_lossy(&out.stdout).trim().to_string();
                println!("{}", s.green());
            }
            _ => println!("{}", "Not found in PATH (Install via python.org or uv)".yellow()),
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
            _ => println!("{}", "Not detected (Optional: winget install Ollama.Ollama)".yellow()),
        }

        println!("{}", "================================================================================".cyan());
        println!("{}", "[V] AI WORKSTATION DIAGNOSTICS COMPLETED.".green().bold());
        Ok(())
    }
}
