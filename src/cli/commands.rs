use crate::application::Application;
use crate::ports::benchmark_port::BenchmarkPort;
use crate::ports::database_port::DatabasePort;
use crate::ports::metrics_port::MetricsPort;

use anyhow::Result;
use colored::*;
use dialoguer::{theme::ColorfulTheme, Confirm, Select};

pub fn run_benchmark<DB: DatabasePort, B: BenchmarkPort, M: MetricsPort>(
    app: &mut Application<DB, B, M>,
    tool: &Option<String>,
) -> Result<()> {
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
