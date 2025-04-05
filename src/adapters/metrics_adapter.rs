use crate::ports::metrics_port::MetricsPort;
use anyhow::Result;
use std::collections::HashMap;

pub struct MetricsAdapter {
    metrics: HashMap<String, f64>,
}

impl MetricsAdapter {
    pub fn new() -> Self {
        Self {
            metrics: HashMap::new(),
        }
    }
}

impl MetricsPort for MetricsAdapter {
    fn collect_memoryinfo(&self) -> Result<serde_json::Value> {
        Ok(serde_json::Value::Null)
    }
    fn collect_cpuinfo(&self) -> Result<serde_json::Value> {
        Ok(serde_json::Value::Null)
    }
    fn collect_vmstat(&self) -> Result<serde_json::Value> {
        Ok(serde_json::Value::Null)
    }

    fn collect_mpstat(&self) -> Result<serde_json::Value> {
        Ok(serde_json::Value::Null)
    }
}
