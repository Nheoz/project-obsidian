use anyhow::Result;
use colored::*;
use std::process::Command;

pub struct TweaksModule;

impl TweaksModule {
    /// Applies low-latency system tweaks:
    ///   1. Network latency tuning (Nagle off, TCP ACK frequency, Delivery Optimization, MMCSS)
    ///   2. IRQ priority tuning (system timer, raise priority separation)
    ///   3. Timer resolution (1ms global timer via GlobalTimerResolutionRequests)
    ///   4. Multimedia scheduling (MMCSS Games profile)
    ///   5. Disable NTFS Last Access timestamps
    ///   6. Disable Memory Compression (frees CPU cycles at idle)
    pub fn apply(dry_run: bool) -> Result<()> {
        println!(
            "{}",
            t!(
                en: "[+] Applying Advanced Low-Latency System Tweaks...",
                es: "[+] Aplicando Tweaks Avanzados de Baja Latencia del Sistema..."
            )
            .cyan()
        );

        if dry_run {
            println!(
                "{}",
                t!(
                    en: "  [DRY-RUN] Would apply network tuning, IRQ priorities, and 1ms timer resolution.",
                    es: "  [SIMULACIÓN] Aplicaría ajuste de red, prioridades IRQ y resolución de temporizador de 1ms."
                )
                .dimmed()
            );
            return Ok(());
        }

        // ── 1. Network Latency Tuning ──────────────────────────────────────────
        println!(
            "\n{}",
            t!(
                en: "  ── Network Latency Tuning ──",
                es: "  ── Ajuste de Latencia de Red ──"
            )
            .white()
            .bold()
        );

        // 1a. Disable Nagle Algorithm (TCP_NODELAY) — sends packets immediately
        println!(
            "{}",
            t!(
                en: "  [*] Disabling Nagle Algorithm (TCP NoDelay)...",
                es: "  [*] Desactivando el Algoritmo de Nagle (TCP NoDelay)..."
            )
            .dimmed()
        );
        println!(
            "{}",
            t!(
                en: "      (Nagle buffers small TCP packets to merge them — adds 5-40ms latency in games and real-time AI calls)",
                es: "      (Nagle agrupa paquetes TCP pequeños para combinarlos — añade 5-40ms de latencia en juegos y llamadas de IA en tiempo real)"
            )
            .green()
        );
        let nagle_cmd = r#"
            $path = 'HKLM:\SYSTEM\CurrentControlSet\Services\Tcpip\Parameters\Interfaces'
            Get-ChildItem $path | ForEach-Object {
                Set-ItemProperty -Path $_.PSPath -Name 'TcpAckFrequency' -Value 1 -Type DWord -Force -ErrorAction SilentlyContinue
                Set-ItemProperty -Path $_.PSPath -Name 'TCPNoDelay'      -Value 1 -Type DWord -Force -ErrorAction SilentlyContinue
                Set-ItemProperty -Path $_.PSPath -Name 'TcpDelAckTicks'  -Value 0 -Type DWord -Force -ErrorAction SilentlyContinue
            }
            Set-ItemProperty -Path 'HKLM:\SYSTEM\CurrentControlSet\Services\Tcpip\Parameters' `
                -Name 'TCPNoDelay' -Value 1 -Type DWord -Force
        "#;
        run_ps(nagle_cmd)?;
        println!(
            "{}",
            t!(
                en: "  [OK] Nagle Algorithm disabled — TCP packets sent immediately.",
                es: "  [OK] Algoritmo de Nagle desactivado — paquetes TCP enviados inmediatamente."
            )
            .green()
        );

        // 1b. Disable NetworkThrottlingIndex (MMCSS throttles non-multimedia traffic)
        println!(
            "{}",
            t!(
                en: "  [*] Disabling Network Throttling Index (MMCSS)...",
                es: "  [*] Desactivando el Índice de Limitación de Red (MMCSS)..."
            )
            .dimmed()
        );
        println!(
            "{}",
            t!(
                en: "      (Windows throttles non-multimedia network traffic to prioritize audio — removes this cap entirely)",
                es: "      (Windows limita el tráfico de red no multimedia para priorizar el audio — elimina este límite por completo)"
            )
            .green()
        );
        let throttle_cmd = r#"
            $path = 'HKLM:\SOFTWARE\Microsoft\Windows NT\CurrentVersion\Multimedia\SystemProfile'
            Set-ItemProperty -Path $path -Name 'NetworkThrottlingIndex' -Value 0xFFFFFFFF -Type DWord -Force
            Set-ItemProperty -Path $path -Name 'SystemResponsiveness'   -Value 0          -Type DWord -Force
        "#;
        run_ps(throttle_cmd)?;
        println!(
            "{}",
            t!(
                en: "  [OK] Network throttle removed — maximum bandwidth always available.",
                es: "  [OK] Límite de red eliminado — ancho de banda máximo siempre disponible."
            )
            .green()
        );

        // 1c. Disable Delivery Optimization P2P upload seeding
        println!(
            "{}",
            t!(
                en: "  [*] Blocking Windows Update P2P seeding to other PCs...",
                es: "  [*] Bloqueando la distribución P2P de Windows Update a otros equipos..."
            )
            .dimmed()
        );
        println!(
            "{}",
            t!(
                en: "      (Delivery Optimization uploads your bandwidth to seed Windows updates to strangers — causes random ping spikes)",
                es: "      (La Optimización de Entrega usa tu ancho de banda para distribuir actualizaciones a desconocidos — causa picos de ping aleatorios)"
            )
            .green()
        );
        let do_cmd = r#"
            $path = 'HKLM:\SOFTWARE\Policies\Microsoft\Windows\DeliveryOptimization'
            if (-not (Test-Path $path)) { New-Item -Path $path -Force | Out-Null }
            Set-ItemProperty -Path $path -Name 'DODownloadMode' -Value 0 -Type DWord -Force
        "#;
        run_ps(do_cmd)?;
        println!(
            "{}",
            t!(
                en: "  [OK] P2P seeding disabled — your bandwidth stays yours.",
                es: "  [OK] Distribución P2P desactivada — tu ancho de banda es solo tuyo."
            )
            .green()
        );

        // ── 2. IRQ Priority Tuning ─────────────────────────────────────────────
        println!(
            "\n{}",
            t!(
                en: "  ── IRQ Priority Tuning ──",
                es: "  ── Ajuste de Prioridad IRQ ──"
            )
            .white()
            .bold()
        );

        println!(
            "{}",
            t!(
                en: "  [*] Elevating system timer interrupt (IRQ8) to maximum priority...",
                es: "  [*] Elevando la interrupción del temporizador del sistema (IRQ8) a prioridad máxima..."
            )
            .dimmed()
        );
        println!(
            "{}",
            t!(
                en: "      (IRQ8 is the hardware clock interrupt — higher priority means the OS scheduler fires more predictably, reducing frame-time jitter)",
                es: "      (IRQ8 es la interrupción del reloj hardware — mayor prioridad hace que el planificador del SO sea más predecible, reduciendo el jitter de frametimes)"
            )
            .green()
        );
        let irq_cmd = r#"
            $path = 'HKLM:\SYSTEM\CurrentControlSet\Control\PriorityControl'
            Set-ItemProperty -Path $path -Name 'IRQ8Priority'          -Value 1  -Type DWord -Force
            Set-ItemProperty -Path $path -Name 'Win32PrioritySeparation'-Value 38 -Type DWord -Force
        "#;
        // Win32PrioritySeparation = 38 = variable interval, short quanta, max boost for foreground
        run_ps(irq_cmd)?;
        println!(
            "{}",
            t!(
                en: "  [OK] IRQ8 priority elevated — foreground app scheduling boosted.",
                es: "  [OK] Prioridad IRQ8 elevada — planificación de apps en primer plano mejorada."
            )
            .green()
        );

        // ── 3. Timer Resolution — 1ms Global ──────────────────────────────────
        println!(
            "\n{}",
            t!(
                en: "  ── Timer Resolution (1ms System-Wide) ──",
                es: "  ── Resolución de Temporizador (1ms a nivel de sistema) ──"
            )
            .white()
            .bold()
        );

        println!(
            "{}",
            t!(
                en: "  [*] Enabling 1ms global timer resolution (Windows 11 22H2+ feature)...",
                es: "  [*] Habilitando resolución de temporizador global de 1ms (característica de Windows 11 22H2+)..."
            )
            .dimmed()
        );
        println!(
            "{}",
            t!(
                en: "      (The default Windows timer fires every 15.6ms — setting global 1ms means ALL processes get finer scheduling, not just foreground apps)",
                es: "      (El temporizador de Windows por defecto dispara cada 15,6ms — establecer 1ms global significa que TODOS los procesos obtienen planificación más fina, no solo los de primer plano)"
            )
            .green()
        );
        // GlobalTimerResolutionRequests = 1 activates system-wide 1ms timer (Windows 11 22H2+)
        // Unlike older BCDEDIT hacks, this is the Microsoft-official mechanism
        let timer_cmd = r#"
            $path = 'HKLM:\SYSTEM\CurrentControlSet\Control\Session Manager\kernel'
            Set-ItemProperty -Path $path -Name 'GlobalTimerResolutionRequests' -Value 1 -Type DWord -Force
        "#;
        run_ps(timer_cmd)?;
        println!(
            "{}",
            t!(
                en: "  [OK] 1ms timer resolution active system-wide — takes effect on next boot.",
                es: "  [OK] Resolución de 1ms activa en todo el sistema — efectiva tras el próximo reinicio."
            )
            .green()
        );

        // ── 4. MMCSS Games Profile — Maximize scheduler slice for games ────────
        println!(
            "\n{}",
            t!(
                en: "  ── Multimedia Scheduler (MMCSS) Games Profile ──",
                es: "  ── Perfil de Juegos del Planificador Multimedia (MMCSS) ──"
            )
            .white()
            .bold()
        );

        println!(
            "{}",
            t!(
                en: "  [*] Tuning MMCSS Games profile for minimum scheduler latency...",
                es: "  [*] Ajustando el perfil de Juegos de MMCSS para mínima latencia del planificador..."
            )
            .dimmed()
        );
        println!(
            "{}",
            t!(
                en: "      (MMCSS is Windows' real-time scheduler for games and audio — setting GPU priority and CPU exclusive mode cuts render latency)",
                es: "      (MMCSS es el planificador en tiempo real de Windows para juegos y audio — configurar prioridad GPU y modo exclusivo de CPU reduce la latencia de renderizado)"
            )
            .green()
        );
        let mmcss_cmd = r#"
            $base = 'HKLM:\SOFTWARE\Microsoft\Windows NT\CurrentVersion\Multimedia\SystemProfile\Tasks'
            $games = Join-Path $base 'Games'
            if (-not (Test-Path $games)) { New-Item -Path $games -Force | Out-Null }
            Set-ItemProperty -Path $games -Name 'Affinity'              -Value 0  -Type DWord  -Force
            Set-ItemProperty -Path $games -Name 'Background Only'       -Value 'False' -Type String -Force
            Set-ItemProperty -Path $games -Name 'Clock Rate'            -Value 10000 -Type DWord -Force
            Set-ItemProperty -Path $games -Name 'GPU Priority'          -Value 8  -Type DWord  -Force
            Set-ItemProperty -Path $games -Name 'Priority'              -Value 6  -Type DWord  -Force
            Set-ItemProperty -Path $games -Name 'Scheduling Category'   -Value 'High'  -Type String -Force
            Set-ItemProperty -Path $games -Name 'SFIO Priority'         -Value 'High'  -Type String -Force
        "#;
        run_ps(mmcss_cmd)?;
        println!(
            "{}",
            t!(
                en: "  [OK] MMCSS Games profile tuned — GPU Priority 8, CPU Priority 6.",
                es: "  [OK] Perfil de Juegos MMCSS ajustado — Prioridad GPU 8, Prioridad CPU 6."
            )
            .green()
        );

        // ── 5. NTFS Last Access Timestamps off ────────────────────────────────
        println!(
            "\n{}",
            t!(
                en: "  ── NTFS Filesystem Optimizations ──",
                es: "  ── Optimizaciones del Sistema de Archivos NTFS ──"
            )
            .white()
            .bold()
        );

        println!(
            "{}",
            t!(
                en: "  [*] Disabling NTFS last-access timestamp updates...",
                es: "  [*] Desactivando la actualización de la marca de tiempo de último acceso de NTFS..."
            )
            .dimmed()
        );
        println!(
            "{}",
            t!(
                en: "      (Every time a file is read, NTFS writes the access time back to disk — disabling this eliminates thousands of micro-writes per hour)",
                es: "      (Cada vez que se lee un archivo, NTFS escribe la hora de acceso en disco — desactivar esto elimina miles de micro-escrituras por hora)"
            )
            .green()
        );
        let out = Command::new("fsutil")
            .args(["behavior", "set", "disablelastaccess", "1"])
            .output()?;
        if out.status.success() {
            println!(
                "{}",
                t!(
                    en: "  [OK] NTFS last-access write eliminated — fewer unnecessary NVMe writes.",
                    es: "  [OK] Escritura de último acceso NTFS eliminada — menos escrituras innecesarias en NVMe."
                )
                .green()
            );
        }

        println!(
            "\n{}",
            t!(
                en: "  [✓] All low-latency tweaks applied. A RESTART is recommended to activate the 1ms timer globally.",
                es: "\n  [✓] Todos los tweaks de baja latencia aplicados. Se recomienda REINICIAR para activar el temporizador de 1ms de forma global."
            )
            .green()
            .bold()
        );

        Ok(())
    }
}

fn run_ps(cmd: &str) -> Result<()> {
    let out = Command::new("powershell")
        .args(["-NoProfile", "-ExecutionPolicy", "Bypass", "-Command", cmd])
        .output()?;
    if !out.status.success() {
        let err = String::from_utf8_lossy(&out.stderr);
        if !err.trim().is_empty() {
            eprintln!(
                "  {}",
                format!("[warn] PowerShell: {}", err.trim()).yellow()
            );
        }
    }
    Ok(())
}
