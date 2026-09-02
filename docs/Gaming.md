# Gaming Optimization & Anti-Cheat Compatibility

Obsidian maximizes frame pacing and 1% lows by eliminating background interruptions. 

## Zero Anti-Cheat Interference
Unlike extreme debloaters, Project Obsidian does **not** disable critical kernel components like PatchGuard, HVCI, or core networking services (RPC/DCOM). This ensures 100% compatibility with:
- Easy Anti-Cheat (EAC)
- BattlEye
- Riot Vanguard (Valorant / LoL)
- Call of Duty Ricochet

## Implemented Optimizations (v2.0)

### 1. Enforcing Windows Game Mode
- **Action:** Forces AllowAutoGameMode and AutoGameModeEnabled in the GameBar registry.
- **Why:** Tells the Windows thread scheduler to heavily prioritize the active foreground game window, preventing background apps from stealing CPU cycles.

### 2. Disabling Background Game DVR
- **Action:** Sets AppCaptureEnabled and GameDVR_Enabled to 0.
- **Why:** Windows 11 defaults to constantly recording your screen in a rolling buffer for the "Record That" feature. This causes constant NVMe writes and steals GPU NVENC/VCE encoding resources, leading to micro-stutters. Obsidian disables the background recording overhead while keeping Xbox Live / Party Chat fully functional.
