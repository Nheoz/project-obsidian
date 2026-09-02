use crate::benchmark::BenchmarkMetrics;
use crate::hardware::HardwareInfo;
use crate::validation::ValidationCheck;
use crate::windows::WindowsInfo;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComprehensiveReport {
    pub timestamp: String,
    pub windows: WindowsInfo,
    pub hardware: HardwareInfo,
    pub applied_profile: String,
    pub validation_checks: Vec<ValidationCheck>,
    pub benchmark: Option<BenchmarkMetrics>,
}

impl ComprehensiveReport {
    pub fn generate_markdown(&self) -> String {
        let mut md = String::new();
        md.push_str("# Project Obsidian — Execution & Optimization Report\n\n");
        md.push_str(&format!("**Generated:** {}\n", self.timestamp));
        md.push_str(&format!(
            "**Applied Profile:** {}\n\n",
            self.applied_profile
        ));

        md.push_str("## 1. System Baseline\n");
        md.push_str(&format!(
            "- **OS:** {} (Build {})\n",
            self.windows.caption, self.windows.build_number
        ));
        md.push_str(&format!(
            "- **Processor:** {} ({} Cores)\n",
            self.hardware.cpu_brand, self.hardware.cpu_cores
        ));
        md.push_str(&format!(
            "- **System Memory:** {:.2} GB\n",
            self.hardware.total_memory_gb
        ));
        for gpu in &self.hardware.gpus {
            md.push_str(&format!(
                "- **Graphics:** {} (Driver {})\n",
                gpu.name, gpu.driver_version
            ));
        }
        md.push_str("\n");

        md.push_str("## 2. Validation & Subsystem Health\n\n");
        md.push_str("| Component | Category | Status | Details |\n");
        md.push_str("| :--- | :--- | :--- | :--- |\n");
        for v in &self.validation_checks {
            let icon = if v.passed { "HEALTHY" } else { "FAILED" };
            md.push_str(&format!(
                "| {} | {} | {} | {} |\n",
                v.component, v.category, icon, v.details
            ));
        }
        md.push_str("\n");

        if let Some(b) = &self.benchmark {
            md.push_str("## 3. Performance Metrics\n\n");
            md.push_str(&format!(
                "- **RAM in Use:** {:.2} GB ({:.1}%)\n",
                b.used_memory_gb, b.memory_used_percent
            ));
            md.push_str(&format!("- **Active Processes:** {}\n", b.total_processes));
            md.push_str(&format!("- **Active Threads:** {}\n", b.total_threads));
            md.push_str(&format!(
                "- **CPU Idle Load:** {:.1}%\n\n",
                b.cpu_global_usage_percent
            ));
        }

        md.push_str("## 4. Rollback Readiness\n\n");
        md.push_str("An atomic snapshot has been recorded under `obsidian-state/`.\n");
        md.push_str("To revert any modifications at any time, execute:\n");
        md.push_str("```powershell\nobsidian restore\n# or\n.\\Restore-Obsidian.ps1\n```\n");

        md
    }

    pub fn save_all(&self, base_path: &Path) -> Result<()> {
        let md = self.generate_markdown();
        fs::write(base_path, md)?;
        let json_path = base_path.with_extension("json");
        let json = serde_json::to_string_pretty(self)?;
        fs::write(json_path, json)?;
        Ok(())
    }
}
