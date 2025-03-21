use anyhow::Result;

pub trait KeyValueStore {
    fn get(&self, key: &str) -> Result<Option<String>>;
    fn set(&mut self, key: &str, value: &str) -> Result<()>;
    fn delete(&mut self, key: &str) -> Result<()>;
}

pub struct EmbeddedStore;

impl EmbeddedStore {
    pub fn new() -> Self {
        EmbeddedStore
    }
}

impl KeyValueStore for EmbeddedStore {
    // Implement the methods (for now, just return dummy values)
    fn get(&self, _key: &str) -> Result<Option<String>> {
        Ok(None)
    }
    fn set(&mut self, _key: &str, _value: &str) -> Result<()> {
        Ok(())
    }
    fn delete(&mut self, _key: &str) -> Result<()> {
        Ok(())
    }
}
