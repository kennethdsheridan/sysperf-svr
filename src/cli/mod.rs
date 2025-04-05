mod cli_struct;
mod commands;

use std::sync::Arc;
use self::cli_struct::{Cli, Commands};
use crate::ports::{
    database_port::DatabasePort,
    metrics_port::MetricsPort,
    benchmark_port::BenchmarkPort,
    log_port::LoggerPort,
};
use crate::adapters::{
    database_adapter::DatabaseAdapter,
    metrics_adapter::MetricsAdapter,
    benchmark_adapter::BenchmarkAdapter,
    log_adapter::init,
};
use crate::application::Application;
use anyhow::Result;
use clap::Parser;
use colored::*;
use dialoguer::{theme::ColorfulTheme, Confirm, Select};
use log::LevelFilter;

pub fn run() -> Result<()> {
    let cli = Cli::parse();

    let log_level = match cli.debug {
        0 => LevelFilter::Info,
        1 => LevelFilter::Debug,
        _ => LevelFilter::Trace,
    };

    // Create logger
    let logger: Arc<dyn LoggerPort> = Arc::new(
        init("logs", log_level)
            .map_err(|e| anyhow::anyhow!("Failed to initialize logger: {}", e))?,
    );
    
    let logger_clone = logger.clone();
    logger_clone.log_info("CLI application starting...");
    if cli.debug > 0 {
        logger_clone.log_debug(&format!("Debug mode enabled (level: {})", cli.debug.to_string()));
    }
   
    // Create adapters as trait objects
    let db: Arc<dyn DatabasePort> = Arc::new(DatabaseAdapter::new());
    let benchmark: Arc<dyn BenchmarkPort> = Arc::new(BenchmarkAdapter::new(
        String::from("fio"),
        vec![String::from("--version")],
        logger.clone(),
    ));
    let metrics: Arc<dyn MetricsPort> = Arc::new(MetricsAdapter::new());

    // Create application with port interfaces
    let mut app = Application::new(
        db,
        benchmark,
        metrics,
        logger
    );

    match &cli.command {
        Some(Commands::Benchmark { tool }) => {
            app.logger.log_info(&format!("Running benchmark with tool: {}", tool.as_deref().unwrap_or("default")));
            commands::run_benchmark(&mut app, tool)?;
        }
        Some(Commands::Collect { metric }) => {
            app.logger.log_info(&format!("Collecting metrics: {}", metric.as_deref().unwrap_or("default")));
            commands::collect_metrics(&mut app, metric)?;
        }
        None => {
            app.logger.log_info("Starting interactive mode");
            commands::run_interactive(&mut app)?;
        }
    }

    app.logger.log_info("CLI application completed successfully");
    Ok(())
}

