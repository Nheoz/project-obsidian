# AI Workstation Diagnostics & Runtimes

Project Obsidian configures your Windows 11 system for optimal local AI inference and development, ensuring long-running tasks don't crash and memory is properly allocated.

## Implemented Optimizations (v2.0)

### 1. Hardware-Accelerated GPU Scheduling (HAGS)
- **Registry Key:** HKLM:\SYSTEM\CurrentControlSet\Control\GraphicsDrivers\HwSchMode (Value: 2)
- **Why:** Offloads GPU scheduling from the CPU to a dedicated hardware-based scheduling processor on the GPU. Reduces latency for CUDA and DirectML workloads.

### 2. TDR Delay Extension (Timeout Detection and Recovery)
- **Registry Keys:** TdrDelay and TdrDdiDelay set to 60 seconds.
- **Why:** By default, Windows restarts the graphics driver if a GPU operation takes longer than 2 seconds. Large AI models (LLMs, Stable Diffusion) can easily block the GPU for longer than 2 seconds, causing crashes. This extends the timeout to 60s.

### 3. NVIDIA Telemetry Container
- **Action:** Disables the NvTelemetryContainer service.
- **Why:** Frees up background threads and system RAM by preventing NVIDIA drivers from sending analytics data to NVIDIA servers.

### 4. WSL2 Resource Boundaries
- **Action:** Creates ~/.wslconfig (8GB RAM, 4 vCPUs, 4GB Swap) if WSL is installed.
- **Why:** WSL2's default dynamic memory allocation can balloon and consume all host memory during large dataset processing. This applies a safe upper bound.
