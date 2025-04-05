use crate::application::Application;
use crate::ports::benchmark_port::{
    BenchmarkConfig, BenchmarkMetrics, BenchmarkParams, BenchmarkTool, FIOParams, IOType,
};
use anyhow::Result;
use colored::*;
use dialoguer::{theme::ColorfulTheme, Confirm, Select};

const BLOCK_SIZES: &[&str] = &[
    "4k", "8k", "16k", "32k", "64k", "128k", "256k", "512k", "1m",
];
const BENCHMARK_DIR: &str = "/mnt/benchmark";

pub fn run_benchmark(app: &mut Application, tool: &Option<String>) -> Result<()> {
    let logger = app.logger.clone();
    logger.log_info("Running benchmark...");

    match tool {
        Some(t) if t.to_lowercase() == "fio" => run_fio_benchmark(app),
        Some(t) => {
            let error_msg = format!("Unsupported benchmark tool: {}", t);
            println!("{}", error_msg);
            logger.log_error(&error_msg);
            Ok(())
        }
        None => {
            let error_msg = "No benchmark tool specified. Use --tool fio to run FIO benchmarks.";
            println!("{}", error_msg);
            logger.log_error(error_msg);
            Ok(())
        }
    }
}

fn run_fio_benchmark(app: &mut Application) -> Result<()> {
    // First validate FIO is available
    if let Err(e) = app.benchmark.run_fio() {
        println!("❌ FIO is not available: {}", e.to_string().red());
        return Ok(());
    }

    println!("Running FIO benchmarks with multiple block sizes...");

    // Validate benchmark directory exists
    if let Err(e) = app.benchmark.validate() {
        println!(
            "❌ Benchmark directory validation failed: {}",
            e.to_string().red()
        );
        return Ok(());
    }

    for &bs in BLOCK_SIZES {
        println!("\nRunning benchmark with block size: {}", bs.blue());

        let config = BenchmarkConfig {
            name: format!("fio_test_{}", bs),
            tool: BenchmarkTool::FIO,
            params: BenchmarkParams::FIO(FIOParams {
                directory: BENCHMARK_DIR.to_string(),
                block_size: bs.to_string(),
                io_type: IOType::RandomRead,
                size: "1G".to_string(),
                runtime: 30,
                num_jobs: 4,
                io_depth: 32,
            }),
        };

        match app.benchmark.run() {
            Ok(_) => {
                println!("✓ Block size {} completed successfully", bs.green());

                // Collect and display system metrics
                if let Ok(cpu_info) = app.metrics.collect_cpuinfo() {
                    println!("  CPU Usage: {:#?}", cpu_info);
                }
                if let Ok(memory_info) = app.metrics.collect_memoryinfo() {
                    println!("  Memory Usage: {:#?}", memory_info);
                }
                if let Ok(vmstat) = app.metrics.collect_vmstat() {
                    println!("  VM Stats: {:#?}", vmstat);
                }
            }
            Err(e) => {
                println!("✗ Block size {} failed: {}", bs, e);
                app.logger
                    .log_error(&format!("Benchmark failed for block size {}: {}", bs, e));
            }
        }
    }

    println!("\nAll FIO benchmarks completed!");
    Ok(())
}

pub fn run_interactive(app: &mut Application) -> Result<()> {
    let options = vec!["FIO Benchmark", "System Metrics", "Exit"];

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select an operation")
        .items(&options)
        .default(0)
        .interact()?;

    match selection {
        0 => run_fio_benchmark(app),
        1 => collect_metrics(app, &None),
        _ => Ok(()),
    }
}

pub fn collect_metrics(app: &mut Application, metric: &Option<String>) -> Result<()> {
    println!("Collecting metrics...");
    Ok(())
}

