mod cli_struct;
mod commands;

use std::sync::Arc;
use self::cli_struct::{Cli, Commands};
use crate::adapters::database_adapter::DatabaseAdapter;
use crate::adapters::log_adapter::{init, FernLogger};
use crate::adapters::{benchmark_adapter::BenchmarkAdapter, metrics_adapter::MetricsAdapter};
use crate::application::Application;
use crate::ports::log_port::LoggerPort;
use anyhow::Result;
use clap::Parser;
use colored::*;
use dialoguer::{theme::ColorfulTheme, Confirm, Select};
use log::logger;
use log::LevelFilter;

pub fn run() -> Result<()> {
    let cli = Cli::parse();

    // Set log level based on debug flag
    let log_level = match cli.debug {
        0 => LevelFilter::Info,
        1 => LevelFilter::Debug,
        _ => LevelFilter::Trace,
    };

    // Initialize the logger using LoggerPort
    let logger: Arc<dyn LoggerPort> = Arc::new(
        init("logs", log_level)
            .map_err(|e| anyhow::anyhow!("Failed to initialize logger: {}", e))?,
    );
    
    logger.log_info("CLI application starting...");
    if cli.debug > 0 {
        logger.log_debug(&format!("Debug mode enabled (level: {})", cli.debug.to_string()));
    }
   
    // Initialize adapters
    let db = DatabaseAdapter::new();
    let benchmark = BenchmarkAdapter::new(
        String::from("fio"),
        vec![String::from("--version")],
        logger.clone(),
    );
    let metrics = MetricsAdapter::new();

    // Create application instance
    let mut app = Application::new(db, benchmark, metrics, logger.clone());

    // Handle subcommands
    match &cli.command {
        Some(Commands::Benchmark { tool }) => {
            logger.as_ref().log_info(&format!("Running benchmark with tool: {}", tool.as_deref().unwrap_or("default")));
            commands::run_benchmark(&mut app, tool)?;
        }
        Some(Commands::Collect { metric }) => {
            logger.as_ref().log_info(&format!("Collecting metrics: {}", metric.as_deref().unwrap_or("default")));
            commands::collect_metrics(&mut app, metric)?;
        }
        None => {
            logger.as_ref().log_info("Starting interactive mode");
            commands::run_interactive(&mut app)?;
        }
    }

    logger.as_ref().log_info("CLI application completed successfully");
    Ok(())
}

