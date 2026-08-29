use serde::{Deserialize, Serialize};
use std::process::Command;
use sysinfo::System;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HardwareInfo {
    pub cpu_brand: String,
    pub cpu_cores: usize,
    pub total_memory_gb: f64,
    pub available_memory_gb: f64,
    pub gpus: Vec<GpuInfo>,
    pub is_nvidia: bool,
    pub is_amd_cpu: bool,
    pub is_intel_cpu: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpuInfo {
    pub name: String,
    pub driver_version: String,
    pub is_nvidia: bool,
}

impl HardwareInfo {
    pub fn detect() -> Self {
        let mut sys = System::new_all();
        sys.refresh_all();

        let cpu_brand = sys
            .cpus()
            .first()
            .map(|c| c.brand().to_string())
            .unwrap_or_else(|| "Unknown Processor".to_string());

        let cpu_cores = sys.cpus().len();
        let total_memory_gb = (sys.total_memory() as f64) / (1024.0 * 1024.0 * 1024.0);
        let available_memory_gb = (sys.available_memory() as f64) / (1024.0 * 1024.0 * 1024.0);

        let is_amd_cpu = cpu_brand.to_lowercase().contains("amd");
        let is_intel_cpu = cpu_brand.to_lowercase().contains("intel");

        let gpus = Self::detect_gpus();
        let is_nvidia = gpus.iter().any(|g| g.is_nvidia);

        HardwareInfo {
            cpu_brand,
            cpu_cores,
            total_memory_gb,
            available_memory_gb,
            gpus,
            is_nvidia,
            is_amd_cpu,
            is_intel_cpu,
        }
    }

    fn detect_gpus() -> Vec<GpuInfo> {
        let mut list = Vec::new();
        // Query WMI via PowerShell CIM
        let output = Command::new("powershell")
            .args([
                "-NoProfile",
                "-Command",
                "Get-CimInstance Win32_VideoController | Select-Object Name, DriverVersion | ConvertTo-Json",
            ])
            .output();

        if let Ok(out) = output {
            let json_str = String::from_utf8_lossy(&out.stdout);
            if let Ok(val) = serde_json::from_str::<serde_json::Value>(&json_str) {
                if let Some(arr) = val.as_array() {
                    for item in arr {
                        let name = item["Name"].as_str().unwrap_or("").to_string();
                        let driver = item["DriverVersion"].as_str().unwrap_or("").to_string();
                        let is_nv = name.to_lowercase().contains("nvidia") || name.to_lowercase().contains("geforce");
                        list.push(GpuInfo {
                            name,
                            driver_version: driver,
                            is_nvidia: is_nv,
                        });
                    }
                } else if val.is_object() {
                    let name = val["Name"].as_str().unwrap_or("").to_string();
                    let driver = val["DriverVersion"].as_str().unwrap_or("").to_string();
                    let is_nv = name.to_lowercase().contains("nvidia") || name.to_lowercase().contains("geforce");
                    list.push(GpuInfo {
                        name,
                        driver_version: driver,
                        is_nvidia: is_nv,
                    });
                }
            }
        }
        list
    }
}
