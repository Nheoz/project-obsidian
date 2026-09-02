<div align="center">

<img src="assets/banner.svg" alt="Project Obsidian Banner" width="100%" />

<br/>

[![CI Rust](https://img.shields.io/badge/CI-Rust-orange?logo=rust)](https://github.com/Nheoz/project-obsidian/actions)
[![CI PowerShell](https://img.shields.io/badge/CI-PowerShell-5391FE?logo=powershell)](https://github.com/Nheoz/project-obsidian/actions)
[![Release](https://img.shields.io/github/v/release/Nheoz/project-obsidian?logo=github&color=10b981)](https://github.com/Nheoz/project-obsidian/releases)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![Windows 11](https://img.shields.io/badge/Target-Windows%2011%20(23H2%20--%2025H2)-0078D4?logo=windows)](https://microsoft.com/windows)
[![Architecture](https://img.shields.io/badge/Arch-x86__64-informational)](https://github.com/Nheoz/project-obsidian)
[![Rust Core](https://img.shields.io/badge/Core-Rust%201.85+-orange?logo=rust)](https://www.rust-lang.org/)
[![PowerShell Enterprise](https://img.shields.io/badge/Shell-PowerShell%205.1%20%2F%207+-5391FE?logo=powershell)](https://learn.microsoft.com/powershell/)

<p align="center">
  <b>Forge Windows 11 into a Privacy-First AI & Gaming Workstation.</b><br/>
  <i>Zero Placebos • Anti-Cheat Safe • AI-Ready • Guaranteed 1-Click Rollback</i>
</p>

[Quick Start](#-quick-start) •
[Architecture](#-hybrid-architecture) •
[Profiles](#-profiles) •
[Zero Placebo Policy](#-zero-placebo-policy) •
[Benchmark Engine](#-empirical-benchmarking) •
[Rollback](#-disaster-recovery--rollback) •
[Documentation](#-technical-documentation)

---

</div>

## 🌌 Overview

**Project Obsidian** is an open-source, hybrid enterprise suite (compiled Rust Core + PowerShell Administration Layer) engineered to transform stock Windows 11 into a lean, privacy-respecting, high-performance workstation for **Gamers** and **AI Engineers**.

Unlike conventional debloaters that recklessly purge AppX packages, break Windows Update, or introduce game-breaking placebo tweaks from 2012, Project Obsidian operates strictly under enterprise systems engineering principles:

$$\mathbf{SECURITY} > \mathbf{STABILITY} > \mathbf{COMPATIBILITY} > \mathbf{PRIVACY} > \mathbf{TWEAKS}$$

### 🛡️ Non-Negotiable Core Guarantees:
- ✅ **Windows Update Remains 100% Functional**: Security patches and cumulative quality rollouts are never blocked.
- ✅ **Microsoft Defender Antivirus Remains Fully Operational**: Core security protections are never lowered for fake benchmark gains.
- ✅ **Anti-Cheat Safe**: Whitelisted compatibility for Easy Anti-Cheat (EAC), BattlEye, Riot Vanguard, Ricochet, and Blizzard Warden.
- ✅ **AI & Virtualization Ready**: Zero interference with WSL2, Docker Desktop, CUDA compilers, or Python model runtimes.
- ✅ **100% Reversible**: Every single modification is backed up into atomic snapshots (`obsidian-state/`) with instant rollback.

### ✨ What's New in v2.0
- **Total Transparency**: The CLI now explains *exactly* what each tweak does in simple English before applying it.
- **Empirical Benchmarking**: Real-time Before/After metrics delta comparing RAM, CPU, Processes, and active Threads.
- **Enhanced Safety**: New interactive warnings before destructive operations and deep integrity checks post-rollback.
- **Robustness**: Background CI testing (Rust `cargo test`, PowerShell `PSScriptAnalyzer`) and persistent `obsidian.log` diagnostic traces.

---

## ⚡ Quick Start

### Option 1: Standalone Binary (Recommended)
Download the latest pre-compiled `obsidian.exe` from [Releases](https://github.com/Nheoz/project-obsidian/releases), open PowerShell as **Administrator**, and run:

```powershell
# 1. Audit your system without making ANY changes (Dry-Run)
.\obsidian.exe analyze

# 2. Inspect your AI stack (NVIDIA, CUDA, WSL2, Docker, Python)
.\obsidian.exe doctor

# 3. Capture baseline performance benchmark
.\obsidian.exe benchmark --label baseline

# 4. Apply the Ultimate Workstation profile
.\obsidian.exe apply --profile ultimate

# 5. Run post-flight zero-breakage validation
.\obsidian.exe validate
```

### Option 2: Pure PowerShell (No Rust binary required)
If you prefer running directly from source without compiling Rust:

```powershell
# Audit system posture
.\Obsidian.ps1 -Command Doctor

# Apply optimizations safely
.\Obsidian.ps1 -Command Apply -Profile Ultimate

# Rollback any time to Windows defaults
.\Restore-Obsidian.ps1
```

---

## 🏗️ Hybrid Architecture

```text
┌─────────────────────────────────────────────────────────────────┐
│                      PROJECT OBSIDIAN CLI                       │
│                         (obsidian.exe)                          │
└────────────────────────────────┬────────────────────────────────┘
                                 │
                 ┌───────────────┴───────────────┐
                 ▼                               ▼
  ┌─────────────────────────────┐ ┌─────────────────────────────┐
  │         RUST CORE           │ │     POWERSHELL LAYER        │
  │                             │ │                             │
  │ • Hardware Probe (NVAPI/WMI)│ │ • Safe Typed Registry       │
  │ • AI Health Doctor (CUDA)   │ │ • Services Policy Guard     │
  │ • Kernel Benchmark Engine   │ │ • Scheduled Tasks Toggle    │
  │ • Atomic Snapshot Engine    │ │ • Group Policy Nodes        │
  │ • Rollback Transaction Mgr  │ │ • Zero-Breakage Validation  │
  └─────────────────────────────┘ └─────────────────────────────┘
                 │                               │
                 └───────────────┬───────────────┘
                                 ▼
         ┌───────────────────────────────────────────────┐
         │         WINDOWS 11 KERNEL & SUBSYSTEMS        │
         │       (23H2 / 24H2 / 25H2 • AMD & Intel)      │
         └───────────────────────────────────────────────┘
```

---

## 🎯 Profiles

| Profile | Target Audience | Key Modifications |
| :--- | :--- | :--- |
| **`Privacy`** | General Users & Workstations | Disables diagnostic telemetry, Advertising ID, Bing search in Start, Copilot, Widgets, and CEIP tasks. |
| **`Gaming`** | Competitive Gamers & Creators | Enforces Game Mode, keeps HAGS/VRR active, disables background Game DVR capture overhead, preserves anti-cheats. |
| **`AI`** | AI Researchers & ML Engineers | Audits NVIDIA RTX, Tensor Cores, CUDA Toolkit, WSL2 Linux kernel, Docker daemon, Python, and Ollama. |
| **`Developer`** | Software Engineers | Validates Git, Windows Terminal, VS Code, and Winget. |
| **`Ultimate`** | Modern Power Users | Harmonious synthesis of all profiles above with full validation. |

---

## 🚫 Zero Placebo Policy

| Tweak Idea | Obsidian's Decision | Technical Justification |
| :--- | :--- | :--- |
| **BCD / Timer Resolution Hacks** | ❌ **REJECTED** | Destroys modern timer tick virtualization, causes micro-stutter in DirectX 12, and desyncs multi-CCD CPUs. |
| **Disabling Windows Defender** | ❌ **REJECTED** | Never degrade operating system defenses for synthetic point gains. |
| **Disabling RPC / WMI / BITS** | ❌ **REJECTED** | Disabling these causes systemic Windows instability, installer crashes, and unbootable states. |
| **Aggressive AppX Package Purge** | ❌ **REJECTED** | Removing inbox UWP dependencies corrupts shell components and breaks the Microsoft Store. |
| **Disabling Telemetry (`DiagTrack`)** | ✅ **APPLIED** | Safely cuts background telemetry transmission to Microsoft data collection endpoints. |
| **Disabling Bing Search in Start** | ✅ **APPLIED** | Keeps Start menu searches purely local, making searches instant and private. |
| **Disabling Game DVR Overhead** | ✅ **APPLIED** | Eliminates constant background disk writes during gameplay without breaking Xbox party/login. |

---

## 📈 Realistic Performance Gains & Concrete User Benefits

Project Obsidian rejects vague marketing buzzwords. Every performance gain is grounded in real Windows kernel behavior and measurable resource liberation:

| Area | Before Obsidian | After Obsidian | Measurable User Benefit |
| :--- | :--- | :--- | :--- |
| **Idle Memory (RAM)** | Background telemetry agents, Edge webview hosts, and consumer bloat idle at ~18–20 GB. | ~600 MB to 1.5 GB of physical RAM reclaimed immediately. | More dedicated RAM headroom for large local LLM contexts (Ollama/PyTorch) and memory-heavy game engines. |
| **Gaming Frame Pacing (1% Lows)** | `CompatTelRunner.exe` and CEIP tasks trigger unpredictable background CPU and disk scans mid-game. | Zero background compatibility thrashing; Game Mode prioritized. | Drastically smoother frametime delivery and elimination of random micro-stutters in competitive games (Valorant, CS2, WoW, Apex). |
| **Game DVR Overhead** | Windows continuously encodes background gameplay video to NVMe storage. | Background video encoding disabled (`AppCaptureEnabled = 0`). | Zero background GPU/CPU video encoder utilization and zero write queue congestion during gameplay. |
| **Network & In-Game Ping** | Windows Delivery Optimization (P2P) uploads update blocks to other PCs over your internet connection. | P2P seeding blocked (`DODownloadMode = 0`); HTTP-only downloads retained. | Eliminates sudden in-game latency spikes, packet jitter, and bandwidth saturation. |
| **NVMe SSD Endurance** | Windows continuously writes ETW telemetry traces, error reporting dumps, and CEIP logs to disk. | Telemetry logging daemons stopped (`DiagTrack`, `WerSvc` disabled). | Extends the TBW (Total Bytes Written) lifespan of high-speed NVMe/PCIe SSD drives. |
| **Start Menu & Shell Speed** | Typing in the Start Menu queries Bing web servers over the internet before showing local files. | Purely local Windows Search indexing (`DisableWebSearch = 1`). | Start Menu results appear instantaneously with zero keystroke data transmitted to external servers. |
| **AI Developer Headroom** | Background telemetry threads contend with WSL2 virtual machine memory ballooning and Docker containers. | Leaner kernel thread pool (-30 to -50 background threads). | Maximum compute threads and VRAM available for local model inference and CUDA development. |

---

## 📊 Empirical Benchmarking

Obsidian includes an empirical benchmark engine that queries the Windows Kernel directly. **Zero synthetic claims, zero marketing exaggerations**. In version 2.0, Obsidian automatically calculates the real-time delta between your pre-flight and post-flight state:

```powershell
.\obsidian.exe apply --profile ultimate
```

Generated metrics report during application:
```text
================================================================================
PROJECT OBSIDIAN — BEFORE vs AFTER COMPARISON
================================================================================
  RAM In Use                : 18.55 → 17.82 GB  [-0.73 ↓]
  Active Processes          : 354 → 312  [-42 ↓]
  Active Threads            : 3500 → 3120  [-380 ↓]
  CPU Usage                 : 3.2% → 1.8%  [-1.40 ↓]
================================================================================
```

---

## 🔄 Disaster Recovery & Rollback

Obsidian enforces atomic reversibility. Before any system change is performed:
1. An atomic JSON snapshot is captured under `obsidian-state/snapshot-YYYYMMDD-HHMMSS.json`.
2. Previous values and types of every registry entry are recorded.
3. Original service startup types and scheduled task states are preserved.

To restore your machine back to its exact previous state:
```powershell
# Via Rust CLI:
.\obsidian.exe restore

# Or via Standalone PowerShell:
.\Restore-Obsidian.ps1
```

---

## 📚 Technical Documentation

Explore in-depth technical documentation in the [`docs/`](docs/) directory:
- [**Privacy Hardening Reference**](docs/Privacy.md)
- [**Gaming Optimization & Anti-Cheat Compatibility**](docs/Gaming.md)
- [**AI Workstation Diagnostics & Runtimes**](docs/AI.md)
- [**Developer Tooling Architecture**](docs/Developer.md)
- [**Services Classification Matrix**](docs/Services.md)
- [**Telemetry & UTC Deep Dive**](docs/Telemetry.md)
- [**Rollback & State Recovery**](docs/Rollback.md)
- [**Security Model & Trust Boundaries**](docs/Security.md)
- [**Benchmark Methodology**](docs/Benchmarks.md)

---

## ⚖️ Honest Privacy Disclosure
Project Obsidian limits and disables all optional diagnostic data, advertising profiles, typing samples, and promotional telemetry. However, we do not claim that Windows has "zero telemetry," as Microsoft retains mandatory operational telemetry required for servicing and cryptographic security verification. Anyone claiming to achieve "0% telemetry" on Windows 11 without breaking Windows Update is making false claims.

---

## 🤝 Contributing & License
Contributions are warmly welcomed! Please read [CONTRIBUTING.md](CONTRIBUTING.md) and [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md).

Project Obsidian is licensed under the [MIT License](LICENSE).
