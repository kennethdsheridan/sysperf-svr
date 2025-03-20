use log::LevelFilter; // you can still import macros if you want
use sysperf_svr_adapters::log_adapter::{FernLogger, init};
use sysperf_svr_ports::log_port::LoggerPort;

fn main() {
    let logger = init("./logs", LevelFilter::Info).expect("Failed to initialize logger");

    // Call the trait methods:
    logger.log_info("This is an info-level log message via LoggerPort");
    logger.log_warn("This is a warning-level log message via LoggerPort");

    println!("CLI logic is running...");
}
