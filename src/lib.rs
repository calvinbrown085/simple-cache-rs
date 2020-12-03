use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;
use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
struct Entry<V> {
    value: V,
    insert_time: Instant,
}

#[derive(Debug, Clone)]
pub struct SimpleCache<K, V> {
    hashmap: HashMap<K, Entry<V>>,
    timeout: Option<Duration>,
}

impl<K: Eq + Hash + Clone + Debug, V: Clone + Debug> SimpleCache<K, V> {
    /// Returns a new instance of SimpleCache
    ///
    /// ```
    /// use simple_cache_rs::SimpleCache;
    ///
    /// let mut cache: SimpleCache<i32, String> = SimpleCache::new(None);
    /// ```
    /// OR
    /// ```
    /// use simple_cache_rs::SimpleCache;
    /// use std::time::Duration;
    ///
    /// let timeout = Duration::new(5, 0);
    /// let mut cache: SimpleCache<i32, String> = SimpleCache::new(Some(timeout));
    /// ```
    pub fn new(timeout: Option<Duration>) -> SimpleCache<K, V> {
        SimpleCache {
            hashmap: HashMap::new(),
            timeout,
        }
    }

    /// Get a value optionally from the cache, if the value is expired this method will return None
    /// and delete the value lazily from the cache.
    /// ```
    /// use simple_cache_rs::SimpleCache;
    ///
    /// let mut cache: SimpleCache<i32, String> = SimpleCache::new(None);
    ///
    /// cache.get(&1);
    /// ```
    pub fn get(&mut self, key: &K) -> Option<V> {
        let entry = self.hashmap.get(key)?;

        if let Some(timeout) = self.timeout {
            if entry.insert_time.elapsed() >= timeout {
                self.delete(key);
                return None;
            }
        }

        Some(entry.value.clone())
    }

    /// Get all keys that are in the cache
    /// ```
    /// use simple_cache_rs::SimpleCache;
    ///
    /// let mut cache: SimpleCache<i32, String> = SimpleCache::new(None);
    ///
    /// cache.keys();
    /// ```
    pub fn keys(&self) -> Vec<K> {
        self.hashmap.keys().map(|k| k.clone()).collect::<Vec<K>>()
    }

    /// Get all values that are in the cache
    /// ```
    /// use simple_cache_rs::SimpleCache;
    ///
    /// let mut cache: SimpleCache<i32, String> = SimpleCache::new(None);
    ///
    /// cache.values();
    /// ```
    pub fn values(&self) -> Vec<V> {
        self.hashmap
            .values()
            .map(|v| v.value.clone())
            .collect::<Vec<V>>()
    }

    /// Insert a batch of items into the cache
    /// ```
    /// use simple_cache_rs::SimpleCache;
    ///
    /// let mut cache: SimpleCache<i32, String> = SimpleCache::new(None);
    ///
    /// let items = vec!((1, String::from("a")), (2, String::from("b")));
    /// cache.insert_batch(items);
    /// ```
    pub fn insert_batch(&mut self, items: Vec<(K, V)>) {
        let i_now = Instant::now();

        for item in items {
            self.hashmap.insert(
                item.0,
                Entry {
                    value: item.1,
                    insert_time: i_now,
                },
            );
        }
    }

    /// Insert an item into the cache
    /// ```
    /// use simple_cache_rs::SimpleCache;
    ///
    /// let mut cache: SimpleCache<i32, String> = SimpleCache::new(None);
    ///
    /// cache.insert(1, String::from("a"));
    /// ```
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        self.hashmap
            .insert(
                key,
                Entry {
                    value,
                    insert_time: Instant::now(),
                },
            )
            .map(|entry| entry.value)
    }

    /// Remove an entry from the cache
    /// ```
    /// use simple_cache_rs::SimpleCache;
    ///
    /// let mut cache: SimpleCache<i32, String> = SimpleCache::new(None);
    ///
    /// cache.insert(1, String::from("a"));
    /// cache.delete(&1);
    /// ```
    pub fn delete(&mut self, key: &K) -> Option<V> {
        self.hashmap.remove(key).map(|entry| entry.value)
    }
}

#[cfg(test)]
mod tests {
    use super::SimpleCache;
    use std::{thread, time::Duration};

    #[test]
    fn insert_and_get_item() {
        let mut scache: SimpleCache<i32, String> = SimpleCache::new(None);
        scache.insert(1, String::from("hello"));

        let v = scache.get(&1);
        assert_eq!(Some(String::from("hello")), v)
    }

    #[test]
    fn insert_and_get_item_and_remove() {
        let mut scache: SimpleCache<i32, String> = SimpleCache::new(None);
        scache.insert(1, String::from("hello"));

        let v = scache.get(&1);
        assert_eq!(Some(String::from("hello")), v);

        scache.delete(&1);

        let no_value = scache.get(&1);
        assert_eq!(None, no_value)
    }

    #[test]
    fn insert_batch_test() {
        let mut scache: SimpleCache<i32, String> = SimpleCache::new(None);

        scache.insert_batch(vec![(1, String::from("hello")), (2, String::from("world"))]);

        let values = scache.values();

        assert!(values.contains(&&String::from("hello")));
        assert!(values.contains(&&String::from("world")))
    }

    #[test]
    fn get_keys_test() {
        let mut scache: SimpleCache<i32, String> = SimpleCache::new(None);
        scache.insert(1, String::from("hello"));

        let keys = scache.keys();
        assert_eq!(keys, vec!(1))
    }

    #[test]
    fn get_values_test() {
        let mut scache: SimpleCache<i32, String> = SimpleCache::new(None);
        scache.insert(1, String::from("hello"));

        let values = scache.values();
        assert_eq!(values, vec!("hello"))
    }

    #[test]
    fn insert_with_timeout() {
        let timeout = Duration::new(1, 0);
        let mut scache: SimpleCache<i32, String> = SimpleCache::new(Some(timeout));

        scache.insert(1, String::from("hello"));
        thread::sleep(Duration::new(1, 1));

        let v = scache.get(&1);
        assert_eq!(None, v)
    }
}
