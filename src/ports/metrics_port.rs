use anyhow::Result;
use serde_json::Value;

/// Port interface for system metrics collection in the hexagonal architecture.
///
/// This trait defines a standardized interface for collecting various system performance
/// metrics. It abstracts the underlying implementation details of metrics collection,
/// allowing for different collection mechanisms while maintaining a consistent API.
///
/// # Design Philosophy
/// - Separates metrics collection concerns from business logic
/// - Provides a unified interface for gathering system performance data
/// - Returns structured JSON data for flexible processing and analysis
/// - Handles errors gracefully through Result types
///
/// # Implementation Requirements
/// Implementors must:
/// - Handle collection failures gracefully
/// - Return well-formed JSON data
/// - Implement timeout mechanisms for long-running collections
/// - Ensure thread safety for concurrent metric collection
///
/// # Example Implementation
/// ```rust
/// struct LinuxMetricsCollector;
///
/// impl MetricsPort for LinuxMetricsCollector {
///     fn collect_mpstat(&self) -> Result<Value> {
///         // Collect CPU statistics using mpstat
///         Ok(serde_json::json!({
///             "cpu_user": 25.5,
///             "cpu_sys": 12.3,
///             "cpu_idle": 62.2
///         }))
///     }
/// }
/// ```
pub trait MetricsPort {
    /// Collects CPU utilization metrics using mpstat.
    ///
    /// Gathers detailed CPU statistics including user time, system time,
    /// I/O wait, and idle percentages. The data is returned as a JSON
    /// structure for flexible processing.
    ///
    /// # Returns
    /// - `Ok(Value)` containing CPU metrics in JSON format
    /// - `Err` if metric collection fails
    ///
    /// # JSON Structure
    /// ```json
    /// {
    ///     "cpu_user": 25.5,
    ///     "cpu_sys": 12.3,
    ///     "cpu_iowait": 0.2,
    ///     "cpu_idle": 62.0
    /// }
    /// ```
    fn collect_mpstat(&self) -> Result<Value>;

    /// Collects virtual memory statistics using vmstat.
    ///
    /// Provides metrics about system memory usage, including free memory,
    /// used memory, swap usage, and memory pressure indicators.
    ///
    /// # Returns
    /// - `Ok(Value)` containing memory metrics in JSON format
    /// - `Err` if metric collection fails
    ///
    /// # JSON Structure
    /// ```json
    /// {
    ///     "free_memory": 4096,
    ///     "used_memory": 8192,
    ///     "swap_used": 1024,
    ///     "page_faults": 150
    /// }
    /// ```
    fn collect_vmstat(&self) -> Result<Value>;

    /// Collects detailed CPU information and capabilities.
    ///
    /// Gathers static CPU information including model, cores, frequencies,
    /// and supported features. This information typically doesn't change
    /// during system operation.
    ///
    /// # Returns
    /// - `Ok(Value)` containing CPU information in JSON format
    /// - `Err` if information collection fails
    ///
    /// # JSON Structure
    /// ```json
    /// {
    ///     "model_name": "Intel(R) Core(TM) i7-9750H",
    ///     "cores": 6,
    ///     "threads": 12,
    ///     "base_frequency": 2600,
    ///     "features": ["sse4_2", "avx2", "aes"]
    /// }
    /// ```
    fn collect_cpuinfo(&self) -> Result<Value>;

    /// Collects detailed memory subsystem information.
    ///
    /// Provides comprehensive information about the system's memory configuration,
    /// including total capacity, memory types, and memory controller details.
    ///
    /// # Returns
    /// - `Ok(Value)` containing memory information in JSON format
    /// - `Err` if information collection fails
    ///
    /// # JSON Structure
    /// ```json
    /// {
    ///     "total_memory": 16384,
    ///     "memory_type": "DDR4",
    ///     "channels": 2,
    ///     "speed": 3200
    /// }
    /// ```
    fn collect_memoryinfo(&self) -> Result<Value>;
}

