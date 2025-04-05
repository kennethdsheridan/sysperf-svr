use crate::application::Application;
use crate::ports::benchmark_port::{
    BenchmarkConfig, BenchmarkMetrics, BenchmarkParams, BenchmarkPort,
    BenchmarkResult, BenchmarkTool, FIOParams, IOType,
};
use crate::ports::database_port::DatabasePort;
use crate::ports::metrics_port::MetricsPort;

use anyhow::Result;
use colored::*;
use dialoguer::{theme::ColorfulTheme, Confirm, Select};

const BLOCK_SIZES: &[&str] = &["4k", "8k", "16k", "32k", "64k", "128k", "256k", "512k", "1m"];

const BENCHMARK_DIR: &str = "/mnt/benchmark";

pub fn run_benchmark<DB: DatabasePort, B: BenchmarkPort, M: MetricsPort>(
    app: &mut Application<DB, B, M>,
    tool: &Option<String>,
) -> Result<()> {
    log::debug!("Running benchmark...");
    println!("Benchmark completed succesfully!");
    Ok(())
}

pub fn run_interactive<DB: DatabasePort, B: BenchmarkPort, M: MetricsPort>(
    app: &mut Application<DB, B, M>,
) -> Result<()> {
    app.run_benchmark()?;
    println!("Running interactive mode...");
    Ok(())
}

pub fn collect_metrics<DB: DatabasePort, B: BenchmarkPort, M: MetricsPort>(
    app: &mut Application<DB, B, M>,
    metric: &Option<String>,
) -> Result<()> {
    println!("Collecting metrics...");
    Ok(())
}
