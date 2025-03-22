use crate::database::{EmbeddedStore, KeyValueStore};
use crate::ports::database_port::DatabasePort;

pub struct DatabaseAdapter {
    store: EmbeddedStore,
}

impl DatabaseAdapter {
    pub fn new() -> Self {
        Self {
            store: EmbeddedStore::new(),
        }
    }
}

impl DatabasePort for DatabaseAdapter {}

impl KeyValueStore for DatabaseAdapter {
    fn get(&self, key: &str) -> anyhow::Result<Option<String>> {
        self.store.get(key)
    }

    fn set(&mut self, key: &str, value: &str) -> anyhow::Result<()> {
        self.store.set(key, value)
    }

    fn delete(&mut self, key: &str) -> anyhow::Result<()> {
        self.store.delete(key)
    }
}

