//! Memory information collection and parsing
//!
//! This module provides functionality to collect and parse detailed memory information
//! from the system, using `/proc/meminfo` and cgroup memory statistics on Linux systems.
//! It provides comprehensive information about system memory usage, including physical
//! memory, swap, caches, and cgroup-specific memory metrics.
//!
//! # Example
//!
//! ```rust
//! use sysperf_svr::domain::metrics::meminfo::MemInfoCollector;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let collector = MemInfoCollector::new();
//!     let mem_info = collector.collect().await?;
//!
//!     println!("Total Memory: {} MB", mem_info.total_memory_mb());
//!     println!("Available Memory: {} MB", mem_info.available_memory_mb());
//!     println!("Memory Usage: {:.1}%", mem_info.memory_usage_percentage());
//!
//!     Ok(())
//! }
//! ```

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
// Removed `use std::future::Future;` because it's not used.
use thiserror::Error;
use tokio::fs::File;
use tokio::io::AsyncReadExt;

/// Errors that can occur during memory info collection
#[derive(Debug, Error)]
pub enum MemInfoError {
    /// Error reading memory information
    #[error("Failed to read memory information: {0}")]
    ReadError(String),

    /// Error parsing memory information
    #[error("Failed to parse memory information: {0}")]
    ParseError(String),

    /// Error accessing cgroup information
    #[error("Failed to access cgroup information: {0}")]
    CgroupError(String),
}

/// Memory usage information for a cgroup
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CgroupMemoryInfo {
    /// Total memory usage in bytes
    pub usage_bytes: u64,
    /// Memory working set in bytes
    pub working_set_bytes: u64,
    /// RSS (Resident Set Size) in bytes
    pub rss_bytes: u64,
    /// Cache usage in bytes
    pub cache_bytes: u64,
    /// Swap usage in bytes
    pub swap_bytes: u64,
    /// Memory limit in bytes (0 if unlimited)
    pub limit_bytes: u64,
    /// Total inactive file memory in bytes
    pub inactive_file_bytes: u64,
}

/// Detailed memory information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemInfo {
    /// Total physical memory in bytes
    pub total_memory: u64,
    /// Available memory in bytes
    pub available_memory: u64,
    /// Free memory in bytes
    pub free_memory: u64,
    /// Used memory in bytes
    pub used_memory: u64,
    /// Buffered memory in bytes
    pub buffers: u64,
    /// Cached memory in bytes
    pub cached: u64,
    /// Active memory in bytes
    pub active: u64,
    /// Inactive memory in bytes
    pub inactive: u64,
    /// Total swap space in bytes
    pub swap_total: u64,
    /// Free swap space in bytes
    pub swap_free: u64,
    /// Used swap space in bytes
    pub swap_used: u64,
    /// Dirty memory in bytes
    pub dirty: u64,
    /// Writeback memory in bytes
    pub writeback: u64,
    /// Memory that can be reclaimed in bytes
    pub reclaimable: u64,
    /// Cgroup memory information
    pub cgroups: HashMap<String, CgroupMemoryInfo>,
    /// Low memory watermark in bytes
    pub low_watermark: u64,
    /// High memory watermark in bytes
    pub high_watermark: u64,
    /// Timestamp when the information was collected
    pub timestamp: i64,
}

impl MemInfo {
    /// Returns total memory in megabytes
    pub fn total_memory_mb(&self) -> u64 {
        self.total_memory / 1024 / 1024
    }

    /// Returns available memory in megabytes
    pub fn available_memory_mb(&self) -> u64 {
        self.available_memory / 1024 / 1024
    }

    /// Returns used memory in megabytes
    pub fn used_memory_mb(&self) -> u64 {
        self.used_memory / 1024 / 1024
    }

    /// Returns memory usage percentage
    pub fn memory_usage_percentage(&self) -> f64 {
        (self.used_memory as f64 / self.total_memory as f64) * 100.0
    }

    /// Returns swap usage percentage
    pub fn swap_usage_percentage(&self) -> f64 {
        if self.swap_total == 0 {
            0.0
        } else {
            (self.swap_used as f64 / self.swap_total as f64) * 100.0
        }
    }
}

/// Collector for memory information
#[derive(Debug)]
pub struct MemInfoCollector {
    proc_meminfo_path: String,
    cgroup_memory_path: String,
}

/// Default implementation for MemInfoCollector
impl Default for MemInfoCollector {
    fn default() -> Self {
        Self::new()
    }
}

/// Implementation for MemInfoCollector
impl MemInfoCollector {
    /// Creates a new MemInfoCollector
    pub fn new() -> Self {
        Self {
            proc_meminfo_path: "/proc/meminfo".to_string(),
            cgroup_memory_path: "/sys/fs/cgroup/memory".to_string(),
        }
    }

    /// Collects memory information from the system
    ///
    /// # Returns
    ///
    /// Returns a Result containing MemInfo on success, or
    /// a MemInfoError on failure
    ///
    /// # Errors
    ///
    /// Will return an error if:
    /// - Cannot read /proc/meminfo
    /// - Cannot access cgroup information
    /// - Cannot parse memory information
    pub async fn collect(&self) -> Result<MemInfo, MemInfoError> {
        let meminfo_content = self.read_proc_meminfo().await?;
        let mut mem_info = self.parse_meminfo(&meminfo_content)?;

        // Collect cgroup memory information
        mem_info.cgroups = self.collect_cgroup_info().await?;

        Ok(mem_info)
    }

    /// Reads the contents of /proc/meminfo
    async fn read_proc_meminfo(&self) -> Result<String, MemInfoError> {
        let mut file = File::open(&self.proc_meminfo_path)
            .await
            .map_err(|e| MemInfoError::ReadError(e.to_string()))?;

        let mut content = String::new();
        file.read_to_string(&mut content)
            .await
            .map_err(|e| MemInfoError::ReadError(e.to_string()))?;

        Ok(content)
    }

    /// Parses /proc/meminfo content into structured memory information
    fn parse_meminfo(&self, content: &str) -> Result<MemInfo, MemInfoError> {
        let mut values = HashMap::new();

        for line in content.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() < 2 {
                continue;
            }

            let key = parts[0].trim_end_matches(':');
            let value = parts[1].parse::<u64>().map_err(|e| {
                MemInfoError::ParseError(format!("Failed to parse value for {}: {}", key, e))
            })?;

            // Convert kB to bytes
            values.insert(key.to_string(), value * 1024);
        }

        let get_value = |key: &str| -> u64 { values.get(key).copied().unwrap_or(0) };

        Ok(MemInfo {
            total_memory: get_value("MemTotal"),
            available_memory: get_value("MemAvailable"),
            free_memory: get_value("MemFree"),
            used_memory: get_value("MemTotal") - get_value("MemAvailable"),
            buffers: get_value("Buffers"),
            cached: get_value("Cached"),
            active: get_value("Active"),
            inactive: get_value("Inactive"),
            swap_total: get_value("SwapTotal"),
            swap_free: get_value("SwapFree"),
            swap_used: get_value("SwapTotal") - get_value("SwapFree"),
            dirty: get_value("Dirty"),
            writeback: get_value("Writeback"),
            reclaimable: get_value("SReclaimable"),
            cgroups: HashMap::new(),
            low_watermark: self.get_low_watermark().unwrap_or(0),
            high_watermark: self.get_high_watermark().unwrap_or(0),
            timestamp: chrono::Utc::now().timestamp(),
        })
    }

    /// Collects memory information from cgroups
    async fn collect_cgroup_info(&self) -> Result<HashMap<String, CgroupMemoryInfo>, MemInfoError> {
        let mut cgroups = HashMap::new();
        let cgroup_paths = [
            "system.slice",
            "user.slice",
            "docker.service",
            "kubelet.service",
            "kubepods.slice",
        ];

        for cgroup_name in &cgroup_paths {
            let cgroup_path = format!("{}/{}", self.cgroup_memory_path, cgroup_name);
            if let Ok(info) = self.read_cgroup_memory_info(&cgroup_path).await {
                cgroups.insert(cgroup_name.to_string(), info);
            }
        }

        Ok(cgroups)
    }

    /// Reads memory information for a specific cgroup
    ///
    /// This refactoring removes the problematic closure capturing `&cgroup_path`.
async fn read_cgroup_memory_info(
    &self,
    cgroup_path: &str,
) -> Result<CgroupMemoryInfo, MemInfoError> {
    // Make the path owned so it lives across async boundaries safely.
    let base_path = cgroup_path.to_string();

    // Helper function that reads a file and parses its content into a u64.
    // This ensures the path is fully owned and avoids capturing `&str` across `.await`.
    async fn read_file(path: String) -> Result<u64, MemInfoError> {
        // If the .map_err doesn’t compile due to type inference, add an explicit type, e.g. (|e: std::io::Error| ...).
        let content = tokio::fs::read_to_string(&path)
            .await
            .map_err(|e| MemInfoError::CgroupError(e.to_string()))?;
        content
            .trim()
            .parse::<u64>()
            .map_err(|e| MemInfoError::ParseError(e.to_string()))
    }

    // Construct the full paths, which are now owned strings.
    let usage_bytes =
        read_file(format!("{}/{}", base_path, "memory.usage_in_bytes")).await?;
    let memory_stat = 
        read_file(format!("{}/{}", base_path, "memory.stat")).await?;
    let memsw_usage =
        read_file(format!("{}/{}", base_path, "memory.memsw.usage_in_bytes")).await?;
    let limit_bytes =
        read_file(format!("{}/{}", base_path, "memory.limit_in_bytes")).await?;

    Ok(CgroupMemoryInfo {
        usage_bytes,
        working_set_bytes: usage_bytes.saturating_sub(memory_stat),
        rss_bytes: memory_stat,
        cache_bytes: memory_stat,
        swap_bytes: memsw_usage.saturating_sub(usage_bytes),
        limit_bytes,
        inactive_file_bytes: memory_stat,
    })
}

    /// Gets the system's low memory watermark
    fn get_low_watermark(&self) -> Result<u64, MemInfoError> {
        let watermark = std::fs::read_to_string("/proc/sys/vm/min_free_kbytes")
            .map_err(|e| MemInfoError::ReadError(e.to_string()))?
            .trim()
            .parse::<u64>()
            .map_err(|e| MemInfoError::ParseError(e.to_string()))?;
        Ok(watermark * 1024) // Convert to bytes
    }

    /// Gets the system's high memory watermark
    fn get_high_watermark(&self) -> Result<u64, MemInfoError> {
        // High watermark is typically 2x the low watermark
        self.get_low_watermark().map(|low| low * 2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_meminfo_collector() {
        let collector = MemInfoCollector::new();
        let result = collector.collect().await;
        assert!(result.is_ok());

        let mem_info = result.unwrap();
        assert!(mem_info.total_memory > 0);
        assert!(mem_info.available_memory <= mem_info.total_memory);
        assert!(mem_info.memory_usage_percentage() >= 0.0);
        assert!(mem_info.memory_usage_percentage() <= 100.0);
    }

    #[test]
    fn test_parse_meminfo() {
        let collector = MemInfoCollector::new();
        let sample_content = r#"
MemTotal:       16384000 kB
MemFree:         8192000 kB
MemAvailable:   12288000 kB
Buffers:         1048576 kB
Cached:          4194304 kB
SwapCached:            0 kB
Active:          4194304 kB
Inactive:        2097152 kB
SwapTotal:      16384000 kB
SwapFree:       16384000 kB
Dirty:             4096 kB
Writeback:           0 kB
"#;
        let result = collector.parse_meminfo(sample_content);
        assert!(result.is_ok());

        let mem_info = result.unwrap();
        assert_eq!(mem_info.total_memory, 16384000 * 1024);
        assert_eq!(mem_info.free_memory, 8192000 * 1024);
        assert_eq!(mem_info.swap_total, 16384000 * 1024);
    }
}

