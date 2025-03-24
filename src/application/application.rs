//! Core application implementing the hexagonal architecture pattern.
//! This module serves as the primary orchestrator for the system performance testing application.

use crate::ports::benchmark_port::BenchmarkPort;
use crate::ports::database_port::DatabasePort;
use crate::ports::metrics_port::MetricsPort;

/// Main application struct that coordinates all core functionality.
///
/// # Type Parameters
/// * `DB` - Database adapter type that implements DatabasePort
/// * `B`  - Benchmark adapter type that implements benchmark_port
/// * `M`  - Metrics adapter type that implements MetricsPort
///
/// # Generic Constraints
/// All type parameters must implement their respective port traits, enabling
/// dependency inversion and making the application independent of specific implementations.
pub struct Application<DB, B, M>
where
    DB: DatabasePort,
    B: BenchmarkPort,
    M: MetricsPort,
{
    /// Database adapter for persistence operations
    db: DB,
    /// Benchmark adapter for running performance tests
    benchmark: B,
    /// Metrics adapter for collecting and reporting metrics
    metrics: M,
}

impl<DB, B, M> Application<DB, B, M>
where
    DB: DatabasePort,
    B: BenchmarkPort,
    M: MetricsPort,
{
    /// Creates a new Application instance with the provided adapters.
    ///
    /// # Arguments
    /// * `db` - Database adapter instance
    /// * `benchmark` - Benchmark adapter instance
    /// * `metrics` - Metrics adapter instance
    ///
    /// # Returns
    /// A new Application instance configured with the provided adapters
    pub fn new(db: DB, benchmark: B, metrics: M) -> Self {
        Self {
            db,
            benchmark,
            metrics,
        }
    }
    /// Executes a benchmark operation using the configured benchmark adapter.
    ///
    /// # Returns
    /// * `Ok(())` if the benchmark completes successfully
    /// * `Err(anyhow::Error)` if the benchmark fails
    pub fn run_benchmark(&self) -> anyhow::Result<()> {
        self.benchmark.run()?;
        Ok(())
    }
}
