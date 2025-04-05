use crate::database::{EmbeddedStore, KeyValueStore};
use crate::ports::database_port::DatabasePort;

/// Database adapter that implements the database port interface.
/// This adapter wraps an embedded key-value store and provides
/// basic CRUD operations.
pub struct DatabaseAdapter {
    store: EmbeddedStore,
}

impl DatabaseAdapter {
    /// Creates a new instance of DatabaseAdapter with an embedded store.
    ///
    /// # Returns
    /// A new `DatabaseAdapter` instance initialized with an empty embedded store.
    ///
    /// # Example
    /// ```
    /// let db = DatabaseAdapter::new();
    /// ```
    pub fn new() -> Self {
        Self {
            store: EmbeddedStore::new(),
        }
    }
}

impl DatabasePort for DatabaseAdapter {}

impl KeyValueStore for DatabaseAdapter {
    /// Retrieves a value from the store by its key.
    ///
    /// # Arguments
    /// * `key` - The key to look up in the store
    ///
    /// # Returns
    /// * `Ok(Some(String))` - The value if found
    /// * `Ok(None)` - If the key doesn't exist
    /// * `Err` - If there was an error accessing the store
    fn get(&self, key: &str) -> anyhow::Result<Option<String>> {
        self.store.get(key)
    }

    /// Stores a key-value pair in the store.
    ///
    /// # Arguments
    /// * `key` - The key under which to store the value
    /// * `value` - The value to store
    ///
    /// # Returns
    /// * `Ok(())` - If the operation was successful
    /// * `Err` - If there was an error writing to the store
    fn set(&mut self, key: &str, value: &str) -> anyhow::Result<()> {
        self.store.set(key, value)
    }

    /// Removes a key-value pair from the store.
    ///
    /// # Arguments
    /// * `key` - The key to remove from the store
    ///
    /// # Returns
    /// * `Ok(())` - If the operation was successful
    /// * `Err` - If there was an error deleting from the store
    fn delete(&mut self, key: &str) -> anyhow::Result<()> {
        self.store.delete(key)
    }
}

