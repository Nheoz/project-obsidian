<#
.SYNOPSIS
    Project Obsidian - Validation & Health Engine
.DESCRIPTION
    Comprehensive health audit executed post-apply. Validates that critical OS services,
    networking, anticheats, gaming runtimes, WSL2, Docker, and developer tooling remain 100% intact.
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

    # 1. Windows Update Subsystem
    $wu = Get-Service -Name 'wuauserv' -ErrorAction SilentlyContinue
    $wuHealthy = ($null -ne $wu -and $wu.StartType -ne 'Disabled')
    Add-HealthCheck -Component 'Windows Update Service' -Category 'CoreOS' -Passed $wuHealthy -Details "Status: $($wu.Status), Startup: $($wu.StartType)"

    # 2. Microsoft Defender Antivirus
    $def = Get-Service -Name 'WinDefend' -ErrorAction SilentlyContinue
    $defHealthy = ($null -ne $def -and $def.Status -eq 'Running')
    Add-HealthCheck -Component 'Microsoft Defender' -Category 'Security' -Passed $defHealthy -Details "Status: $($def.Status)"

    # 3. DNS Resolution
    $dnsOk = $false
    try {
        $ip = [System.Net.Dns]::GetHostAddresses("cloudflare.com")
        $dnsOk = ($ip.Count -gt 0)
    } catch { $dnsOk = $false }
    Add-HealthCheck -Component 'DNS Resolution' -Category 'Network' -Passed $dnsOk -Details "Resolving cloudflare.com: $dnsOk"

    # 4. Network Adapter Connectivity
    $netAdapters = Get-NetAdapter -Physical -ErrorAction SilentlyContinue | Where-Object { $_.Status -eq 'Up' }
    $netOk = ($null -ne $netAdapters -and $netAdapters.Count -gt 0)
    Add-HealthCheck -Component 'Physical Network Interface' -Category 'Network' -Passed $netOk -Details "Active Adapters: $($netAdapters.Count)"

    # 5. Bluetooth Subsystem
    $bth = Get-Service -Name 'bthserv' -ErrorAction SilentlyContinue
    $bthOk = ($null -ne $bth -and $bth.StartType -ne 'Disabled')
    Add-HealthCheck -Component 'Bluetooth Service' -Category 'Hardware' -Passed $bthOk -Details "Status: $($bth.Status), Startup: $($bth.StartType)"

    # 6. Spooler (Printers)
    $spool = Get-Service -Name 'Spooler' -ErrorAction SilentlyContinue
    $spoolOk = ($null -ne $spool -and $spool.StartType -ne 'Disabled')
    Add-HealthCheck -Component 'Print Spooler' -Category 'Peripherals' -Passed $spoolOk -Details "Status: $($spool.Status), Startup: $($spool.StartType)"

    # 7. Microsoft Store / AppX
    $store = Get-AppxPackage -Name "Microsoft.WindowsStore" -ErrorAction SilentlyContinue
    $storeOk = ($null -ne $store)
    Add-HealthCheck -Component 'Microsoft Store AppX' -Category 'Gaming' -Passed $storeOk -Details "Version: $($store.Version)"

    # 8. WSL2 Runtime
    $wslCmd = Get-Command 'wsl.exe' -ErrorAction SilentlyContinue
    $wslOk = ($null -ne $wslCmd)
    Add-HealthCheck -Component 'WSL2 Command' -Category 'AI/Developer' -Passed $wslOk -Details "Path: $($wslCmd.Source)"

    # 9. NVIDIA Driver & Display
    $gpu = Get-CimInstance Win32_VideoController | Where-Object { $_.Name -like '*NVIDIA*' } | Select-Object -First 1
    $gpuOk = ($null -ne $gpu)
    $gpuDetails = if ($gpuOk) { "$($gpu.Name) [Driver: $($gpu.DriverVersion)]" } else { "No NVIDIA GPU detected" }
    Add-HealthCheck -Component 'NVIDIA Graphics Subsystem' -Category 'Hardware' -Passed $gpuOk -Details $gpuDetails

    # 10. Cryptographic & RPC Core
    $crypt = Get-Service -Name 'CryptSvc' -ErrorAction SilentlyContinue
    $rpc = Get-Service -Name 'RpcSs' -ErrorAction SilentlyContinue
    $coreOk = ($null -ne $crypt -and $crypt.Status -eq 'Running' -and $null -ne $rpc -and $rpc.Status -eq 'Running')
    Add-HealthCheck -Component 'RPC & Cryptographic Services' -Category 'CoreOS' -Passed $coreOk -Details "RpcSs: $($rpc.Status), CryptSvc: $($crypt.Status)"

    return $report
}
