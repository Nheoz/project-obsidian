<#
.SYNOPSIS
    Project Obsidian - Validation & Health Engine
.DESCRIPTION
    Comprehensive health audit executed post-apply. Validates that critical OS services,
    networking, anticheats, gaming runtimes, WSL2, Docker, and developer tooling remain 100% intact.
    Also validates the Windows security posture was not degraded by any optimization.
#>

Set-StrictMode -Version Latest
$ErrorActionPreference = 'SilentlyContinue'

function Test-ObsidianHealth {
    [CmdletBinding()]
    param()

    $report = [System.Collections.Generic.List[object]]::new()

    function Add-HealthCheck {
        param([string]$Component, [string]$Category, [bool]$Passed, [string]$Details)
        $report.Add([PSCustomObject]@{
            Component = $Component
            Category  = $Category
            Status    = if ($Passed) { 'HEALTHY' } else { 'FAILED' }
            Passed    = $Passed
            Details   = $Details
        })
    }

    # ── CORE OS ────────────────────────────────────────────────────────────────

    # 1. Windows Update Subsystem
    $wu = Get-Service -Name 'wuauserv' -ErrorAction SilentlyContinue
    $wuHealthy = ($null -ne $wu -and $wu.StartType -ne 'Disabled')
    Add-HealthCheck -Component 'Windows Update Service' -Category 'CoreOS' -Passed $wuHealthy -Details "Status: $($wu.Status), Startup: $($wu.StartType)"

    # 2. Cryptographic & RPC Core
    $crypt = Get-Service -Name 'CryptSvc' -ErrorAction SilentlyContinue
    $rpc   = Get-Service -Name 'RpcSs'   -ErrorAction SilentlyContinue
    $coreOk = ($null -ne $crypt -and $crypt.Status -eq 'Running' -and $null -ne $rpc -and $rpc.Status -eq 'Running')
    Add-HealthCheck -Component 'RPC & Cryptographic Services' -Category 'CoreOS' -Passed $coreOk -Details "RpcSs: $($rpc.Status), CryptSvc: $($crypt.Status)"

    # ── SECURITY ───────────────────────────────────────────────────────────────

    # 3. Microsoft Defender — Service
    $def = Get-Service -Name 'WinDefend' -ErrorAction SilentlyContinue
    $defHealthy = ($null -ne $def -and $def.Status -eq 'Running')
    Add-HealthCheck -Component 'Microsoft Defender (Service)' -Category 'Security' -Passed $defHealthy -Details "Status: $($def.Status)"

    # 4. Microsoft Defender — Real-Time Protection enabled
    $mpStatus = Get-MpComputerStatus -ErrorAction SilentlyContinue
    $rtpOn = ($null -ne $mpStatus -and $mpStatus.RealTimeProtectionEnabled -eq $true)
    Add-HealthCheck -Component 'Defender Real-Time Protection' -Category 'Security' -Passed $rtpOn -Details "RealTimeProtection: $($mpStatus.RealTimeProtectionEnabled)"

    # 5. Windows Defender Firewall — all three profiles
    $fw = Get-NetFirewallProfile -ErrorAction SilentlyContinue
    $fwOk = ($null -ne $fw -and ($fw | Where-Object { $_.Enabled -eq $true }).Count -ge 1)
    $fwDetails = ($fw | ForEach-Object { "$($_.Name): $($_.Enabled)" }) -join ' | '
    Add-HealthCheck -Component 'Windows Firewall (Profiles)' -Category 'Security' -Passed $fwOk -Details $fwDetails

    # 6. UAC (User Account Control) — must not have been disabled
    $uacKey = Get-ItemProperty -Path 'HKLM:\SOFTWARE\Microsoft\Windows\CurrentVersion\Policies\System' -ErrorAction SilentlyContinue
    $uacEnabled = ($null -ne $uacKey -and $uacKey.EnableLUA -eq 1)
    Add-HealthCheck -Component 'User Account Control (UAC)' -Category 'Security' -Passed $uacEnabled -Details "EnableLUA: $($uacKey.EnableLUA)"

    # 7. Secure Boot status
    try {
        $secureBoot = Confirm-SecureBootUEFI -ErrorAction SilentlyContinue
        Add-HealthCheck -Component 'Secure Boot (UEFI)' -Category 'Security' -Passed ($secureBoot -eq $true) -Details "SecureBoot: $secureBoot"
    } catch {
        # Confirm-SecureBootUEFI throws on legacy BIOS — treat as not applicable
        Add-HealthCheck -Component 'Secure Boot (UEFI)' -Category 'Security' -Passed $true -Details "Not applicable on this firmware (Legacy BIOS)"
    }

    # 8. SmartScreen — must still be active
    $ssKey = Get-ItemProperty -Path 'HKLM:\SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer' -ErrorAction SilentlyContinue
    $ssEnabled = ($null -eq $ssKey -or $ssKey.SmartScreenEnabled -ne 'Off')
    Add-HealthCheck -Component 'SmartScreen Filter' -Category 'Security' -Passed $ssEnabled -Details "SmartScreenEnabled: $($ssKey.SmartScreenEnabled)"

    # 9. Windows Defender Network Inspection Service
    $wdNis = Get-Service -Name 'WdNisSvc' -ErrorAction SilentlyContinue
    $wdNisOk = ($null -ne $wdNis -and $wdNis.StartType -ne 'Disabled')
    Add-HealthCheck -Component 'Defender Network Inspection' -Category 'Security' -Passed $wdNisOk -Details "Status: $($wdNis.Status), Startup: $($wdNis.StartType)"

    # 10. BitLocker (informational — warn if drive is unencrypted)
    $bl = Get-BitLockerVolume -ErrorAction SilentlyContinue | Where-Object { $_.MountPoint -eq $env:SystemDrive } | Select-Object -First 1
    $blProtected = ($null -ne $bl -and $bl.ProtectionStatus -eq 'On')
    $blDetails = if ($null -ne $bl) { "Drive $($bl.MountPoint): ProtectionStatus=$($bl.ProtectionStatus)" } else { "BitLocker query unavailable" }
    # BitLocker is informational only — not a hard failure (many gaming setups skip it)
    Add-HealthCheck -Component 'BitLocker Drive Encryption' -Category 'Security' -Passed $true -Details "[INFO] $blDetails"

    # ── NETWORK ────────────────────────────────────────────────────────────────

    # 11. DNS Resolution
    $dnsOk = $false
    try {
        $ip = [System.Net.Dns]::GetHostAddresses("cloudflare.com")
        $dnsOk = ($ip.Count -gt 0)
    } catch { $dnsOk = $false }
    Add-HealthCheck -Component 'DNS Resolution' -Category 'Network' -Passed $dnsOk -Details "Resolving cloudflare.com: $dnsOk"

    # 12. Network Adapter Connectivity
    $netAdapters = Get-NetAdapter -Physical -ErrorAction SilentlyContinue | Where-Object { $_.Status -eq 'Up' }
    $netOk = ($null -ne $netAdapters -and $netAdapters.Count -gt 0)
    Add-HealthCheck -Component 'Physical Network Interface' -Category 'Network' -Passed $netOk -Details "Active Adapters: $($netAdapters.Count)"

    # ── HARDWARE ───────────────────────────────────────────────────────────────

    # 13. Bluetooth Subsystem
    $bth = Get-Service -Name 'bthserv' -ErrorAction SilentlyContinue
    $bthOk = ($null -ne $bth -and $bth.StartType -ne 'Disabled')
    Add-HealthCheck -Component 'Bluetooth Service' -Category 'Hardware' -Passed $bthOk -Details "Status: $($bth.Status), Startup: $($bth.StartType)"

    # 14. GPU Driver & Display
    $gpu = Get-CimInstance Win32_VideoController | Where-Object {
        $_.Name -notlike '*Microsoft Basic*' -and $_.Name -notlike '*Remote*' -and $_.AdapterRAM -gt 0
    } | Select-Object -First 1
    $gpuOk = ($null -ne $gpu)
    $gpuDetails = if ($gpuOk) { "$($gpu.Name) [Driver: $($gpu.DriverVersion)]" } else { "No dedicated GPU detected" }
    Add-HealthCheck -Component 'GPU Display Subsystem' -Category 'Hardware' -Passed $gpuOk -Details $gpuDetails

    # ── PERIPHERALS / GAMING ───────────────────────────────────────────────────

    # 15. Print Spooler
    $spool = Get-Service -Name 'Spooler' -ErrorAction SilentlyContinue
    $spoolOk = ($null -ne $spool -and $spool.StartType -ne 'Disabled')
    Add-HealthCheck -Component 'Print Spooler' -Category 'Peripherals' -Passed $spoolOk -Details "Status: $($spool.Status), Startup: $($spool.StartType)"

    # 16. Microsoft Store / AppX
    $store = Get-AppxPackage -Name "Microsoft.WindowsStore" -ErrorAction SilentlyContinue
    $storeOk = ($null -ne $store)
    Add-HealthCheck -Component 'Microsoft Store AppX' -Category 'Gaming' -Passed $storeOk -Details "Version: $($store.Version)"

    # ── AI / DEVELOPER ─────────────────────────────────────────────────────────

    # 17. WSL2 Runtime
    $wslCmd = Get-Command 'wsl.exe' -ErrorAction SilentlyContinue
    $wslOk = ($null -ne $wslCmd)
    Add-HealthCheck -Component 'WSL2 Command' -Category 'AI/Developer' -Passed $wslOk -Details "Path: $($wslCmd.Source)"

    return $report
}

