# Gaming Optimization Architecture

## Zero-Placebo Principles
Project Obsidian rejects dangerous "tweaks" propagated on forums that break modern Windows features:
- **NO BCD Edits**: We do not touch `useplatformclock`, `tscsyncpolicy`, or `disabledynamictick`. These cause desynchronization and micro-stutter on modern multi-core architectures (e.g. AMD 3D V-Cache, Intel Thread Director).
- **NO Timer Resolution Hacks**: Modern Windows scheduler manages timer tick resolution dynamically per-process.
- **NO Core Parking / Power Plan Hijacking**: Modern CPU power schedulers (such as AMD CPPC2) require Windows default scheduler coordination to steer high-priority game threads to V-Cache CCDs or P-cores.
- **NO Security Degradation**: We do NOT disable VBS (Virtualization-Based Security) or Memory Integrity (HVCI) by default.

## Safe, Measurable Optimizations Applied
1. **Windows Game Mode**: Explicitly enforced (`AutoGameModeEnabled = 1`). Prioritizes CPU and GPU resources to the foreground game process.
2. **HAGS & VRR Preserved**: Hardware-Accelerated GPU Scheduling and Variable Refresh Rate settings remain completely active.
3. **Background Game DVR Overhead Removal**:
   - `HKCU:\Software\Microsoft\Windows\CurrentVersion\GameDVR\AppCaptureEnabled = 0`
   - Eliminates continuous background video encoding onto NVMe drives during gameplay.
4. **Intrusive Xbox Overlay Notifications Silenced**:
   - `HKCU:\Software\Microsoft\GameBar\ShowStartupPanel = 0`
   - Preserves Xbox Live login, game invites, and cloud sync while removing annoying popups.
5. **Anti-Cheat Whitelist**:
   - Easy Anti-Cheat (EAC), BattlEye, Riot Vanguard, Ricochet, and Blizzard Warden are 100% untouched and fully operational.
