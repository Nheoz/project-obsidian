# Developer Tooling Architecture

Project Obsidian prepares the OS for software development by removing I/O bottlenecks and ensuring toolchain compatibility.

## Implemented Optimizations (v2.0)

### 1. Win32 Long Path Support
- **Registry Key:** LongPathsEnabled set to 1.
- **Why:** Bypasses the legacy 260-character path limit in Windows. Essential for deep 
ode_modules trees, Rust cargo caches, and Python virtual environments.

### 2. SysMain (Superfetch)
- **Action:** Disables the SysMain service.
- **Why:** Superfetch attempts to pre-load frequently used apps into RAM. During compilation, this causes unnecessary random SSD reads/writes, competing with your compiler for disk I/O.

### 3. NTFS Last Access Time
- **Action:** sutil behavior set disablelastaccess 1
- **Why:** Stops the NTFS file system from writing a timestamp every single time a file is read. Massively speeds up operations that read thousands of files (e.g., git status, cargo build, 
pm install).

### 4. High Performance Power Plan
- **Action:** Sets active plan to 8c5e7fda-e8bf-4a96-9a85-a6e23a8c635c.
- **Why:** Prevents the OS from aggressively throttling CPU clocks during short lulls in compilation, ensuring maximum turbo boost when needed.

### 5. Large System Cache
- **Registry Key:** LargeSystemCache set to 1.
- **Why:** Instructs the Windows Memory Manager to favor the system file cache over application working sets, speeding up subsequent reads of large source code repositories.
