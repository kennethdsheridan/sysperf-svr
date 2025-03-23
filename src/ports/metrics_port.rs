use anyhow::Result;
use serde_json::Value;

pub trait MetricsPort {
    fn collect_mpstat(&self) -> Result<Value>;
    fn collect_vmstat(&self) -> Result<Value>;
    fn collect_cpuinfo(&self) -> Result<Value>;
    fn collect_memoryinfo(&self) -> Result<Value>;
}
