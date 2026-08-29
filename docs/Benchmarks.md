# System Benchmark Methodology

## The Problem with "Placebo" Benchmarks
Many Windows debloater tools claim arbitrary numbers such as *"Increases FPS by 25%"* or *"Cuts input lag by 50%"*. In reality:
- Micro-benchmarks can fluctuate wildly with background thermal throttling and GPU boost clocks.
- Disabling critical kernel security features or timer mechanisms can induce severe micro-stutter and frame-pacing drops in DirectX 12 titles.

## Obsidian's Transparent Empirical Metrics
Project Obsidian measures real, measurable metrics through Windows Kernel APIs:
1. **Physical RAM Footprint**:
   - Total System RAM
   - Active RAM in Use (GB and %)
   - Available Memory
2. **Kernel Process & Thread Count**:
   - Total processes spawned
   - Total active threads tracked across all processes
3. **Idle CPU Utilization**:
   - Baseline CPU percentage load over stable sampling windows
4. **Zero Fabrication**:
   - Metrics are recorded before and after application in `benchmark-pre-apply.json` and `benchmark-post-apply.json`.
   - The comparison report shows exact signed deltas (`-0.85 GB RAM`, `-28 Processes`, `-310 Threads`).
