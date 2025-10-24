use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;

/// Type alias for async callbacks
pub type AsyncCallback<T> =
    Box<dyn Fn(T) -> Pin<Box<dyn Future<Output = ()> + Send>> + Send + Sync>;

/// Callback ID type for identifying callbacks
pub type CallbackId = u64;

/// Generic async callback manager that can handle callbacks for any type T
pub struct AsyncCallbackManager<T> {
    /// Map of callbacks by their ID
    callbacks: HashMap<CallbackId, AsyncCallback<T>>,
    /// Next callback ID to assign
    next_id: CallbackId,
}

impl<T> Default for AsyncCallbackManager<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> AsyncCallbackManager<T> {
    /// Create a new callback manager
    pub fn new() -> Self {
        Self {
            callbacks: HashMap::new(),
            next_id: 0,
        }
    }

    // ----------------------------------------------------------------------------

    /// Generate a new unique callback ID
    fn next_id(&mut self) -> CallbackId {
        let id = self.next_id;
        self.next_id += 1;
        id
    }

    // ----------------------------------------------------------------------------

    /// Add a callback and return its ID
    pub fn add(&mut self, callback: AsyncCallback<T>) -> CallbackId {
        let id = self.next_id();
        self.callbacks.insert(id, callback);
        id
    }

    // ----------------------------------------------------------------------------

    /// Remove a callback by ID
    /// Returns true if the callback was found and removed, false otherwise
    pub fn remove(&mut self, id: CallbackId) -> bool {
        self.callbacks.remove(&id).is_some()
    }

    // ----------------------------------------------------------------------------

    /// Clear all callbacks
    pub fn clear(&mut self) {
        self.callbacks.clear();
    }

    // ----------------------------------------------------------------------------

    /// Get the number of registered callbacks
    pub fn count(&self) -> usize {
        self.callbacks.len()
    }

    // ----------------------------------------------------------------------------

    /// Check if a callback with the given ID exists
    pub fn has_callback(&self, id: CallbackId) -> bool {
        self.callbacks.contains_key(&id)
    }

    // ----------------------------------------------------------------------------

    /// Get all callback IDs
    pub fn callback_ids(&self) -> Vec<CallbackId> {
        self.callbacks.keys().copied().collect()
    }

    // ----------------------------------------------------------------------------

    /// Execute all callbacks with the given data
    pub async fn execute_all_callbacks(&self, data: &T)
    where
        T: Clone,
    {
        for callback in self.callbacks.values() {
            callback(data.clone()).await;
        }
    }

    // ----------------------------------------------------------------------------

    /// Execute a specific callback by ID with the given data
    pub async fn execute_callback(&self, id: CallbackId, data: T) -> bool {
        if let Some(callback) = self.callbacks.get(&id) {
            callback(data).await;
            true
        } else {
            false
        }
    }

    // ----------------------------------------------------------------------------

    /// Get access to the underlying callback map (read-only)
    pub fn callbacks(&self) -> &HashMap<CallbackId, AsyncCallback<T>> {
        &self.callbacks
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};

    #[tokio::test]
    async fn test_callback_manager() {
        let mut manager = AsyncCallbackManager::<i32>::new();
        let counter = Arc::new(Mutex::new(0));

        // Add a callback
        let counter_clone = counter.clone();
        let callback: AsyncCallback<i32> = Box::new(move |value: i32| {
            let counter = counter_clone.clone();
            Box::pin(async move {
                let mut count = counter.lock().unwrap();
                *count += value;
            }) as Pin<Box<dyn Future<Output = ()> + Send>>
        });

        let id = manager.add(callback);

        // Execute the callback
        manager.execute_callback(id, 5).await;

        assert_eq!(*counter.lock().unwrap(), 5);
        assert_eq!(manager.count(), 1);

        // Remove the callback
        assert!(manager.remove(id));
        assert_eq!(manager.count(), 0);
        assert!(!manager.has_callback(id));
    }

    #[tokio::test]
    async fn test_execute_all_callbacks() {
        let mut manager = AsyncCallbackManager::<String>::new();
        let results = Arc::new(Mutex::new(Vec::<String>::new()));

        // Add multiple callbacks
        for i in 1..=3 {
            let results_clone = results.clone();
            let callback: AsyncCallback<String> = Box::new(move |data: String| {
                let results = results_clone.clone();
                let suffix = format!("_callback_{}", i);
                Box::pin(async move {
                    let mut vec = results.lock().unwrap();
                    vec.push(format!("{}{}", data, suffix));
                }) as Pin<Box<dyn Future<Output = ()> + Send>>
            });
            manager.add(callback);
        }

        manager.execute_all_callbacks(&"test".to_string()).await;

        let final_results = results.lock().unwrap();
        assert_eq!(final_results.len(), 3);
        assert!(final_results.contains(&"test_callback_1".to_string()));
        assert!(final_results.contains(&"test_callback_2".to_string()));
        assert!(final_results.contains(&"test_callback_3".to_string()));
    }
}
