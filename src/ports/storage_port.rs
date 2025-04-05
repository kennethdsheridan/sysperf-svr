use anyhow::Result;
use serde_json::Value;

/// Port interface for persistent storage operations in the hexagonal architecture.
///
/// This trait defines a standardized interface for storing and retrieving metrics data
/// in a persistent storage system. It abstracts the underlying storage implementation,
/// allowing for different storage backends (e.g., file system, database, cloud storage)
/// while maintaining a consistent API.
///
/// # Design Philosophy
/// - Provides a simple key-value storage interface
/// - Uses JSON Values for flexible data structure storage
/// - Handles errors through anyhow::Result for comprehensive error handling
/// - Maintains storage implementation independence
///
/// # Implementation Requirements
/// Implementors must:
/// - Ensure thread safety for concurrent storage operations
/// - Handle storage failures gracefully
/// - Maintain data consistency
/// - Implement proper error handling
///
/// # Example Implementation
/// ```rust
/// struct FileSystemStorage {
///     base_path: PathBuf
/// }
///
/// impl StoragePort for FileSystemStorage {
///     fn store_metrics(&self, key: &str, value: &Value) -> Result<()> {
///         let path = self.base_path.join(key);
///         let file = File::create(path)?;
///         serde_json::to_writer(file, value)?;
///         Ok(())
///     }
///
///     fn retrieve_metrics(&self, key: &str) -> Result<Option<Value>> {
///         let path = self.base_path.join(key);
///         if !path.exists() {
///             return Ok(None);
///         }
///         let file = File::open(path)?;
///         let value = serde_json::from_reader(file)?;
///         Ok(Some(value))
///     }
/// }
/// ```
pub trait StoragePort {
    /// Stores metrics data associated with a specific key.
    ///
    /// This method persists the provided JSON value data using the specified key
    /// as an identifier. If data already exists for the given key, it should be
    /// overwritten.
    ///
    /// # Arguments
    /// * `key` - A unique identifier for the metrics data
    /// * `value` - The JSON-formatted metrics data to store
    ///
    /// # Returns
    /// - `Ok(())` if the storage operation succeeds
    /// - `Err` if the storage operation fails
    ///
    /// # Errors
    /// Common error conditions include:
    /// - Storage system unavailable
    /// - Insufficient permissions
    /// - Invalid key format
    /// - Storage capacity exceeded
    fn store_metrics(&self, key: &str, value: &Value) -> Result<()>;

    /// Retrieves metrics data associated with a specific key.
    ///
    /// This method fetches previously stored metrics data using the provided key.
    /// Returns None if no data exists for the given key.
    ///
    /// # Arguments
    /// * `key` - The identifier for the metrics data to retrieve
    ///
    /// # Returns
    /// - `Ok(Some(Value))` if data is found for the key
    /// - `Ok(None)` if no data exists for the key
    /// - `Err` if the retrieval operation fails
    ///
    /// # Errors
    /// Common error conditions include:
    /// - Storage system unavailable
    /// - Insufficient permissions
    /// - Corrupted data
    /// - Invalid key format
    fn retrieve_metrics(&self, key: &str) -> Result<Option<Value>>;
}

