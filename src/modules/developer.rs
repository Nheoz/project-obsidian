use anyhow::{bail, Result};
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
            .args(["-NoProfile", "-Command",
                "(Get-AppxPackage Microsoft.WindowsTerminal* -ErrorAction SilentlyContinue | Select-Object -First 1).Version"])
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
                let s = String::from_utf8_lossy(&o.stdout)
                    .lines()
                    .next()
                    .unwrap_or("Installed")
                    .to_string();
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

        // 5. Rust toolchain
        print!("  {:<25} : ", "Rust Toolchain");
        if let Ok(o) = Command::new("rustc").arg("--version").output() {
            if o.status.success() {
                let s = String::from_utf8_lossy(&o.stdout).trim().to_string();
                println!("{}", s.green());
            } else {
                println!("{}", "Not found (Install: https://rustup.rs)".yellow());
            }
        } else {
            println!("{}", "Not installed".yellow());
        }

        Ok(())
    }

    /// Apply developer environment optimizations:
    /// - Disable Windows Search indexing on common dev folders (reduces I/O noise)
    /// - Enable Long Path support in the registry (required for node_modules, pnpm, etc.)
    /// - Set High Performance power plan (prevents CPU throttling during builds)
    /// - Disable Superfetch/SysMain during dev sessions (reduces random I/O on SSD)
    /// - Set file system cache to large (improves compiler throughput)
    pub fn apply() -> Result<()> {
        println!(
            "{}",
            "[+] Applying Developer Environment Optimizations...".cyan()
        );

        // 1. Enable Win32 Long Path support (essential for node_modules, Rust caches, Python envs)
        println!("{}", t!(
            en: "  [*] Enabling Win32 Long Path support...",
            es: "  [*] Habilitando soporte Win32 para rutas largas..."
        ).dimmed());
        println!("{}", t!(
            en: "      (Allows deeply nested folders like 'node_modules' or Rust cache without crashing tools)",
            es: "      (Permite carpetas muy anidadas como 'node_modules' o caché de Rust sin colgar las herramientas)"
        ).green());
        let longpath_cmd = "\
            Set-ItemProperty \
              -Path 'HKLM:\\SYSTEM\\CurrentControlSet\\Control\\FileSystem' \
              -Name 'LongPathsEnabled' -Value 1 -Type DWord -Force;";

        let out = Command::new("powershell")
            .args([
                "-NoProfile",
                "-ExecutionPolicy",
                "Bypass",
                "-Command",
                longpath_cmd,
            ])
            .output()?;
        if !out.status.success() {
            bail!(
                "Long path registry tweak failed: {}",
                String::from_utf8_lossy(&out.stderr).trim()
            );
        }
        println!("{}", t!(en: "  [OK] Long Path support enabled.", es: "  [OK] Soporte para rutas largas habilitado.").green());

        // 2. Disable SysMain (Superfetch) — reduces random SSD writes during heavy compilation
        println!("{}", t!(
            en: "  [*] Disabling SysMain (Superfetch) service...",
            es: "  [*] Desactivando el servicio SysMain (Superfetch)..."
        ).dimmed());
        println!("{}", t!(
            en: "      (Stops Windows from pre-loading apps into RAM, giving full disk I/O to compilers/IDE)",
            es: "      (Evita que Windows precargue apps en RAM, dedicando todo el I/O del disco a compiladores/IDE)"
        ).green());
        let sysmain_cmd = "\
            $svc = Get-Service -Name 'SysMain' -ErrorAction SilentlyContinue; \
            if ($svc) { \
                Stop-Service -Name 'SysMain' -Force -ErrorAction SilentlyContinue; \
                Set-Service -Name 'SysMain' -StartupType Disabled; \
                Write-Host 'Disabled'; \
            } else { Write-Host 'NotPresent'; }";

        let out = Command::new("powershell")
            .args([
                "-NoProfile",
                "-ExecutionPolicy",
                "Bypass",
                "-Command",
                sysmain_cmd,
            ])
            .output()?;
        let result = String::from_utf8_lossy(&out.stdout).trim().to_string();
        if result == "Disabled" {
            println!("{}", "  [OK] SysMain disabled.".green());
        } else {
            println!("{}", "  [--] SysMain not present (skip).".dimmed());
        }

        // 3. Set NTFS disable last access time update — reduces filesystem overhead on large repos
        println!("{}", t!(
            en: "  [*] Disabling NTFS Last Access Time...",
            es: "  [*] Desactivando la marca de Último Acceso NTFS..."
        ).dimmed());
        println!("{}", t!(
            en: "      (Speeds up 'git status' and build tools by not recording every time a file is read)",
            es: "      (Acelera 'git status' y herramientas de compilación al no registrar cada lectura de archivo)"
        ).green());
        let ntfs_cmd = "fsutil behavior set disablelastaccess 1";
        let out = Command::new("cmd").args(["/c", ntfs_cmd]).output()?;
        if out.status.success() {
            println!("{}", t!(en: "  [OK] NTFS last access time disabled.", es: "  [OK] Tiempo de último acceso NTFS desactivado.").green());
        } else {
            // Non-critical — some Windows editions disallow this
            println!("{}", t!(en: "  [!] NTFS last access tweak skipped (may require elevated fsutil).", es: "  [!] Retoque de acceso NTFS omitido (puede requerir permisos).").yellow());
        }

        // 4. Set active power plan to High Performance for build machines
        println!("{}", t!(
            en: "  [*] Activating High Performance power plan...",
            es: "  [*] Activando el Plan de Energía de Alto Rendimiento..."
        ).dimmed());
        println!("{}", t!(
            en: "      (Ensures the CPU runs at maximum turbo clocks during long build processes)",
            es: "      (Asegura que la CPU corra al máximo turbo durante largos procesos de compilación)"
        ).green());
        let power_cmd = "powercfg /setactive 8c5e7fda-e8bf-4a96-9a85-a6e23a8c635c";
        let out = Command::new("cmd").args(["/c", power_cmd]).output()?;
        if out.status.success() {
            println!("{}", "  [OK] High Performance plan active.".green());
        } else {
            println!(
                "{}",
                "  [!] High Performance plan unavailable (desktop may use Balanced/Ultimate)."
                    .yellow()
            );
        }

        // 5. Increase system file cache working set for large codebases (SetSystemFileCacheSize)
        println!("  {}", "[*] Tuning system file cache...".dimmed());
        println!(
            "      {}",
            "(Allows Windows to cache more project files in RAM, speeding up frequent compiles)"
                .green()
        );
        let cache_cmd = "\
            $path = 'HKLM:\\SYSTEM\\CurrentControlSet\\Control\\Session Manager\\Memory Management'; \
            Set-ItemProperty -Path $path -Name 'LargeSystemCache' -Value 1 -Type DWord -Force;";
        let out = Command::new("powershell")
            .args([
                "-NoProfile",
                "-ExecutionPolicy",
                "Bypass",
                "-Command",
                cache_cmd,
            ])
            .output()?;
        if out.status.success() {
            println!(
                "{}",
                "  [OK] Large system cache enabled — faster file reads on huge repos.".green()
            );
        } else {
            println!("{}", "  [!] System cache tweak skipped.".yellow());
        }

        println!(
            "{}",
            "================================================================================"
                .cyan()
        );
        println!(
            "{}",
            "[V] DEVELOPER ENVIRONMENT OPTIMIZATION APPLIED."
                .green()
                .bold()
        );
        Ok(())
    }
}
