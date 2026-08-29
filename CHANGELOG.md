# Changelog

All notable changes to **Project Obsidian** will be documented in this file.
The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/), and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.0.0] - 2026-08-29
### Added
- **Hybrid Architecture Core**:
  - Rust Core Engine (`obsidian.exe`) providing zero-overhead hardware probing, AI diagnostics, CLI parsing, atomic snapshots, and empirical benchmarking.
  - PowerShell Administration Layer (`powershell/*.ps1`) with typed, auditable, and idempotent Windows registry and service toggling.
- **Atomic Snapshot & Rollback Engine**:
  - Full system state serialization into `obsidian-state/snapshot-*.json`.
  - Native standalone rollback via `Restore-Obsidian.ps1` and CLI `obsidian restore`.
- **Profiles**:
  - `Privacy`: Diagnostic telemetry minimization, Bing start search elimination, advertising ID deactivation, Copilot and Windows Recall neutralization.
  - `Gaming`: Game Mode enforcement, HAGS audit, background Game DVR overhead removal, zero touch on anticheat engines.
  - `AI`: Full diagnostic pipeline (`obsidian doctor`) for NVIDIA GPU, CUDA toolkit, WSL2, Docker daemon, Python, and Ollama.
  - `Developer`: Toolchain availability audit for Windows Terminal, VS Code, Git, and Winget.
  - `Ultimate`: Harmonious combination of all safe profiles.
- **Empirical Benchmarking**:
  - Real-time kernel metric collection (RAM in use, idle CPU load, active process and thread count).
- **GitHub Infrastructure**:
  - GitHub Actions CI workflows for Rust and PowerShell Pester.
  - Standard Issue and Pull Request templates.
  - Full technical architecture documentation under `docs/`.
