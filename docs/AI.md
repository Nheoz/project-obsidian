# AI & Machine Learning Workstation Architecture

## Goal
Prepare Windows 11 as a first-class, low-overhead host for modern local AI inference, LLM fine-tuning, and CUDA-accelerated development.

## Diagnostic Pillars (`obsidian doctor` / `obsidian ai doctor`)
1. **GPU Architecture**: Verifies NVIDIA Ada Lovelace / Blackwell / Ampere architecture, active Driver branch, and dedicated VRAM pool.
2. **CUDA Toolkit & Compiler**: Checks for `nvcc` in system PATH, CUDA versions, and environment variables (`CUDA_PATH`).
3. **WSL2 Virtualization**: Ensures the Linux subsystem is functional for PyTorch/vLLM/DeepSpeed workloads without virtual network isolation issues.
4. **Docker Desktop & Container Daemon**: Audits integration with the WSL2 backend and NVIDIA Container Toolkit.
5. **Local Inference Runtimes**: Detects installed inference runners (Ollama, LM Studio, vLLM, llama.cpp).
6. **Zero Forced Installations**: Project Obsidian *diagnoses and recommends*, but never forcefully installs packages without explicit user intent.
