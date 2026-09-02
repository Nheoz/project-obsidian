# Project Obsidian — Execution & Optimization Report

**Generated:** 2026-09-02T23:29:51.998361200+00:00
**Applied Profile:** Ultimate Workstation

## 1. System Baseline
- **OS:** Microsoft Windows 11 Pro (Build 26200)
- **Processor:** AMD Ryzen 7 7800X3D 8-Core Processor            (16 Cores)
- **System Memory:** 30.88 GB
- **Graphics:** AMD Radeon(TM) Graphics (Driver 32.0.21036.18)
- **Graphics:** NVIDIA GeForce RTX 5070 Ti (Driver 32.0.16.1656)

## 2. Validation & Subsystem Health

| Component | Category | Status | Details |
| :--- | :--- | :--- | :--- |
| Windows Update Service | CoreOS | HEALTHY | Status: Stopped, Startup: Manual |
| Microsoft Defender | Security | HEALTHY | Status: Running |
| DNS Resolution | Network | HEALTHY | Resolving cloudflare.com: True |
| Bluetooth Service | Hardware | HEALTHY | Status: Running, Startup: Manual |
| Print Spooler | Peripherals | HEALTHY | Status: Running, Startup: Automatic |
| Microsoft Store AppX | Gaming | HEALTHY | Version: 22607.1401.7.0 |
| WSL2 Command | AI/Developer | HEALTHY | Path: C:\WINDOWS\system32\wsl.exe |
| GPU Display Subsystem | Hardware | HEALTHY | AMD Radeon(TM) Graphics [Driver: 32.0.21036.18] |
| RPC & Cryptographic Services | CoreOS | HEALTHY | RpcSs: Running, CryptSvc: Running |

## 3. Performance Metrics

- **RAM in Use:** 18.75 GB (60.7%)
- **Active Processes:** 347
- **Active Threads:** 7180
- **CPU Idle Load:** 12.2%

## 4. Rollback Readiness

An atomic snapshot has been recorded under `obsidian-state/`.
To revert any modifications at any time, execute:
```powershell
obsidian restore
# or
.\Restore-Obsidian.ps1
```


---

# Project Obsidian — Benchmark Comparison Report

| Metric | Before | After | Delta | Verdict |
| :--- | :--- | :--- | :--- | :--- |
| **RAM In Use** | 18.76 GB | 18.75 GB | -0.01 GB | ≈ same |
| **Memory Load** | 60.7% | 60.7% | +0.0% | ≈ same |
| **Active Processes** | 345 | 347 | +2 | ↑ higher |
| **Active Threads** | 7166 | 7180 | +14 | ↑ higher |
| **CPU Usage** | 12.1% | 12.2% | +0.1% | ≈ same |

*Captured via Windows Kernel interfaces. Zero synthetic estimation.*
