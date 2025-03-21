use anyhow::Result;
use sysperf_svr::cli;

fn main() -> Result<()> {
    cli::run()?;
    Ok(())
}
