# Security Policy & Trust Model

## Core Tenets
1. **Never Degrade Defenses for Placebo Gains**:
   - Project Obsidian **NEVER** disables Microsoft Defender Antivirus, SmartScreen, or Windows Firewall.
   - Project Obsidian **NEVER** disables Memory Integrity (HVCI) or Virtualization-Based Security (VBS) by default.
2. **Immutable Services**:
   - `wuauserv` (Windows Update), `RpcSs` (RPC), `Winmgmt` (WMI), `BITS`, and `CryptSvc` are strictly protected and blocked from modification by the `Test-IsServiceProtected` policy guard.
3. **Transparent Open Source Auditability**:
   - All PowerShell routines run directly from plaintext `.ps1` scripts without obfuscation.
   - All Rust routines are open-source, compiled under standard MSVC/LLVM toolchains with Cargo.
4. **Vulnerability Reporting**:
   - To report a security vulnerability or unintended breakage, please refer to [SECURITY.md](../SECURITY.md).
