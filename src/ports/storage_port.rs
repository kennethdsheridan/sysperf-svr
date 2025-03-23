use anyhow::Result;
use serde_json::Value;

pub trait StoragePort {
    fn store_metrics(&self, key: &str, value: &Value) -> Result<()>;
    fn retrieve_metrics(&self, key: &str) -> Result<Option<Value>>;
}
