//! CPU information collection and parsing
//!
//! This module provides functionality to collect and parse detailed CPU information
//! from the system, primarily using `/proc/cpuinfo` on Linux systems. It provides
//! comprehensive information about CPU architecture, features, cache sizes, and
//! other hardware characteristics.
//!
//! # Example
//!
//! ```rust
//! use sysperf_svr::domain::metrics::cpuinfo::CpuInfoCollector;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let collector = CpuInfoCollector::new();
//!     let cpu_info = collector.collect().await?;
//!
//!     println!("CPU Model: {}", cpu_info.model_name);
//!     println!("Total Cores: {}", cpu_info.total_cores);
//!     println!("Clock Speed: {} MHz", cpu_info.cpu_mhz);
//!
//!     Ok(())
//! }
//! ```

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;
use tokio::fs::File;
use tokio::io::AsyncReadExt;

/// Errors that can occur during CPU info collection
#[derive(Debug, Error)]
pub enum CpuInfoError {
    /// Error reading /proc/cpuinfo
    #[error("Failed to read CPU information: {0}")]
    ReadError(String),

    /// Error parsing CPU information
    #[error("Failed to parse CPU information: {0}")]
    ParseError(String),
}

/// Cache information for a CPU
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheInfo {
    /// Cache size in KB
    pub size_kb: u32,
    /// Cache level (L1, L2, L3)
    pub level: u8,
    /// Cache type (Data, Instruction, Unified)
    pub cache_type: String,
    /// Ways of associativity
    pub ways: Option<u32>,
}

/// Detailed CPU core information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreInfo {
    /// Physical core ID
    pub core_id: u32,
    /// Physical socket/processor ID
    pub physical_id: u32,
    /// List of logical processor IDs (threads) for this core
    pub processor_ids: Vec<u32>,
    /// Core-specific flags and features
    pub flags: Vec<String>,
    /// Core-specific frequency in MHz
    pub cpu_mhz: f64,
    /// Cache information for this core
    pub caches: HashMap<String, CacheInfo>,
}

/// NUMA node information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NumaInfo {
    /// NUMA node ID
    pub node_id: u32,
    /// CPUs assigned to this NUMA node
    pub cpus: Vec<u32>,
    /// Memory size in bytes
    pub memory_bytes: u64,
    /// Distance to other NUMA nodes (node_id -> distance)
    pub distances: HashMap<u32, u32>,
}

/// Complete CPU information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CpuInfo {
    /// CPU vendor ID
    pub vendor_id: String,
    /// CPU model name
    pub model_name: String,
    /// CPU architecture
    pub architecture: String,
    /// CPU operating mode (32-bit, 64-bit)
    pub op_mode: String,
    /// Total number of CPU cores
    pub total_cores: u32,
    /// Total number of CPU threads
    pub total_threads: u32,
    /// Number of physical sockets/processors
    pub num_sockets: u32,
    /// Base CPU frequency in MHz
    pub cpu_mhz: f64,
    /// Maximum CPU frequency in MHz
    pub max_cpu_mhz: Option<f64>,
    /// Minimum CPU frequency in MHz
    pub min_cpu_mhz: Option<f64>,
    /// Detailed information for each core
    pub cores: HashMap<u32, CoreInfo>,
    /// NUMA node information
    pub numa_nodes: HashMap<u32, NumaInfo>,
    /// CPU flags and features
    pub flags: Vec<String>,
    /// CPU bugs
    pub bugs: Vec<String>,
    /// Additional CPU features
    pub features: HashMap<String, String>,
    /// Timestamp when the information was collected
    pub timestamp: i64,
}

/// Collector for CPU information
#[derive(Debug)]
pub struct CpuInfoCollector {
    proc_cpuinfo_path: String,
}

impl Default for CpuInfoCollector {
    fn default() -> Self {
        Self::new()
    }
}

impl CpuInfoCollector {
    /// Creates a new CpuInfoCollector
    pub fn new() -> Self {
        Self {
            proc_cpuinfo_path: "/proc/cpuinfo".to_string(),
        }
    }

    /// Collects CPU information from the system
    ///
    /// # Returns
    ///
    /// Returns a Result containing CpuInfo on success, or
    /// a CpuInfoError on failure
    ///
    /// # Errors
    ///
    /// Will return an error if:
    /// - Cannot read /proc/cpuinfo
    /// - Cannot parse CPU information
    pub async fn collect(&self) -> Result<CpuInfo, CpuInfoError> {
        let content = self.read_proc_cpuinfo().await?;
        self.parse_cpuinfo(&content)
    }

    /// Reads the contents of /proc/cpuinfo
    async fn read_proc_cpuinfo(&self) -> Result<String, CpuInfoError> {
        let mut file = File::open(&self.proc_cpuinfo_path)
            .await
            .map_err(|e| CpuInfoError::ReadError(e.to_string()))?;

        let mut content = String::new();
        file.read_to_string(&mut content)
            .await
            .map_err(|e| CpuInfoError::ReadError(e.to_string()))?;

        Ok(content)
    }

    /// Parses /proc/cpuinfo content into structured CPU information
    fn parse_cpuinfo(&self, content: &str) -> Result<CpuInfo, CpuInfoError> {
        let mut cpu_info = CpuInfo {
            vendor_id: String::new(),
            model_name: String::new(),
            architecture: String::new(),
            op_mode: String::new(),
            total_cores: 0,
            total_threads: 0,
            num_sockets: 0,
            cpu_mhz: 0.0,
            max_cpu_mhz: None,
            min_cpu_mhz: None,
            cores: HashMap::new(),
            numa_nodes: HashMap::new(),
            flags: Vec::new(),
            bugs: Vec::new(),
            features: HashMap::new(),
            timestamp: chrono::Utc::now().timestamp(),
        };

        let mut current_processor: Option<u32> = None;
        let mut current_core = CoreInfo {
            core_id: 0,
            physical_id: 0,
            processor_ids: Vec::new(),
            flags: Vec::new(),
            cpu_mhz: 0.0,
            caches: HashMap::new(),
        };

        for line in content.lines() {
            if line.trim().is_empty() {
                if let Some(processor) = current_processor {
                    cpu_info.cores.insert(processor, current_core.clone());
                }
                current_processor = None;
                continue;
            }

            let parts: Vec<&str> = line.splitn(2, ':').collect();
            if parts.len() != 2 {
                continue;
            }

            let key = parts[0].trim();
            let value = parts[1].trim();

            match key {
                "processor" => {
                    current_processor = Some(value.parse().map_err(|e| {
                        CpuInfoError::ParseError(format!("Invalid processor ID: {}", e))
                    })?);
                }
                "vendor_id" => cpu_info.vendor_id = value.to_string(),
                "model name" => cpu_info.model_name = value.to_string(),
                "cpu MHz" => {
                    current_core.cpu_mhz = value
                        .parse()
                        .map_err(|e| CpuInfoError::ParseError(format!("Invalid CPU MHz: {}", e)))?;
                }
                "core id" => {
                    current_core.core_id = value
                        .parse()
                        .map_err(|e| CpuInfoError::ParseError(format!("Invalid core ID: {}", e)))?;
                }
                "physical id" => {
                    current_core.physical_id = value.parse().map_err(|e| {
                        CpuInfoError::ParseError(format!("Invalid physical ID: {}", e))
                    })?;
                }
                "flags" => {
                    current_core.flags = value.split_whitespace().map(String::from).collect();
                    cpu_info.flags = current_core.flags.clone();
                }
                "bugs" => {
                    cpu_info.bugs = value.split_whitespace().map(String::from).collect();
                }
                _ => {
                    cpu_info.features.insert(key.to_string(), value.to_string());
                }
            }
        }

        // Calculate total cores and threads
        let unique_core_ids: Vec<u32> = cpu_info
            .cores
            .values()
            .map(|core| core.core_id)
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
            .collect();

        cpu_info.total_cores = unique_core_ids.len() as u32;
        cpu_info.total_threads = cpu_info.cores.len() as u32;
        cpu_info.num_sockets = cpu_info
            .cores
            .values()
            .map(|core| core.physical_id)
            .collect::<std::collections::HashSet<_>>()
            .len() as u32;

        Ok(cpu_info)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_cpuinfo_collector() {
        let collector = CpuInfoCollector::new();
        let result = collector.collect().await;
        assert!(result.is_ok());

        let cpu_info = result.unwrap();
        assert!(!cpu_info.model_name.is_empty());
        assert!(cpu_info.total_cores > 0);
        assert!(cpu_info.total_threads >= cpu_info.total_cores);
    }

    #[test]
    fn test_parse_cpuinfo() {
        let collector = CpuInfoCollector::new();
        let sample_content = r#"
processor       : 0
vendor_id       : GenuineIntel
cpu family      : 6
model           : 142
model name      : Intel(R) Core(TM) i7-8565U CPU @ 1.80GHz
stepping        : 11
microcode       : 0xde
cpu MHz         : 2000.000
cache size      : 8192 KB
physical id     : 0
siblings        : 8
core id         : 0
cpu cores       : 4
apicid          : 0
initial apicid  : 0
fpu             : yes
fpu_exception   : yes
cpuid level     : 22
wp              : yes
flags           : fpu vme de pse tsc msr pae mce cx8 apic sep mtrr pge mca cmov
bugs            : spectre_v1 spectre_v2 spec_store_bypass
"#;
        let result = collector.parse_cpuinfo(sample_content);
        assert!(result.is_ok());

        let cpu_info = result.unwrap();
        assert_eq!(cpu_info.vendor_id, "GenuineIntel");
        assert!(cpu_info.flags.len() > 0);
        assert!(cpu_info.bugs.len() > 0);
    }
}
