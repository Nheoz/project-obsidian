# Benchmark Methodology

Obsidian replaces synthetic, arbitrary scoring systems with empirical, real-time deltas measured directly from the Windows Kernel.

## Real-Time Delta Engine (v2.0)
When you run obsidian apply, the tool captures the system state before and after execution, outputting a direct comparison:

`	ext
================================================================================
PROJECT OBSIDIAN — BEFORE vs AFTER COMPARISON
================================================================================
  RAM In Use                : 18.55 → 17.82 GB  [-0.73 ↓]
  Active Processes          : 354 → 312  [-42 ↓]
  Active Threads            : 3500 → 3120  [-380 ↓]
  CPU Usage                 : 3.2% → 1.8%  [-1.40 ↓]
================================================================================
`

## How Metrics are Gathered
- **RAM & CPU:** Handled via the cross-platform sysinfo crate, querying native Windows performance counters.
- **Thread Count:** Obsidian queries WMI (Win32_Process ThreadCount) directly. (This fixes a known issue where standard Linux process tools fail to count threads accurately on Windows).
- **Process Count:** Enumerated directly from the active session pool.
