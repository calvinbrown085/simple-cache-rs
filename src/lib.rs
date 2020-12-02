use std::collections::HashMap;
use std::hash::Hash;
use std::time::Duration;
use std::time::Instant;

#[derive(Debug, Clone)]
struct ValueWithTimeout<V> {
    value: V,
    insert_time: Instant
}

#[derive(Debug, Clone)]
pub struct SimpleCache<K, V> {
    h: Box<HashMap<K, ValueWithTimeout<V>>>,
    timeout: Option<Duration>,
}

impl<K: Eq + Hash + Clone, V: Clone> SimpleCache<K, V> {

    /// Returns a new instance of SimpleCache
    ///
    /// ```
    /// use simple_cache_rs::SimpleCache;
    ///
    /// let mut cache: SimpleCache<i32, String> = SimpleCache::new();
    ///
    /// ```
    pub fn new() -> SimpleCache<K, V> {
        SimpleCache {
            h: Box::new(HashMap::new()),
            timeout: None,
        }
    }


    /// Returns a new instance of SimpleCache with a timeout for values
    ///
    /// ```
    /// use simple_cache_rs::SimpleCache;
    /// use std::time::Duration;
    ///
    /// let mut cache: SimpleCache<i32, String> = SimpleCache::with_timeout(Duration::new(100, 0));
    ///
    /// ```
    pub fn with_timeout(timeout: Duration) -> SimpleCache<K, V> {
        SimpleCache {
            h: Box::new(HashMap::new()),
            timeout: Some(timeout),
        }
    }


    /// Get a value optionally from the cache, if the value is expired this method will return None
    /// and delete the value lazily from the cache.
    /// ```
    /// use simple_cache_rs::SimpleCache;
    ///
    /// let mut cache: SimpleCache<i32, String> = SimpleCache::new();
    ///
    /// cache.get(&1);
    /// ```
    pub fn get(&self, key: &K) -> Option<V> {
        self.h.get(key).and_then(|v| {
            if let Some(timeout) = self.timeout {
                if v.insert_time.elapsed() >= timeout {
                    self.clone().delete(key);
                    return None
                }
            }
            Some(v.value.clone())
        })
    }

    /// Get all keys that are in the cache
    /// ```
    /// use simple_cache_rs::SimpleCache;
    ///
    /// let mut cache: SimpleCache<i32, String> = SimpleCache::new();
    ///
    /// cache.keys();
    /// ```
    pub fn keys(&self) -> Vec<K> {
        self.h.keys().map(|k| k.clone()).collect::<Vec<K>>()
    }

    /// Get all values that are in the cache
    /// ```
    /// use simple_cache_rs::SimpleCache;
    ///
    /// let mut cache: SimpleCache<i32, String> = SimpleCache::new();
    ///
    /// cache.values();
    /// ```
    pub fn values(&self) -> Vec<V> {
        self.h.values().map(|vwt| vwt.value.clone()).collect::<Vec<V>>()
    }


    /// Insert a batch of items into the cache
    /// ```
    /// use simple_cache_rs::SimpleCache;
    ///
    /// let mut cache: SimpleCache<i32, String> = SimpleCache::new();
    ///
    /// let items = vec!((1, String::from("a")), (2, String::from("b")));
    /// cache.insert_batch(items);
    /// ```
    pub fn insert_batch(&mut self, items: Vec<(K, V)>) {
        for item in items {
            self.h.insert(
                item.0,
                ValueWithTimeout {
                    value: item.1,
                    insert_time: Instant::now()
                },
            );
        }
    }

    /// Insert an item into the cache
    /// ```
    /// use simple_cache_rs::SimpleCache;
    ///
    /// let mut cache: SimpleCache<i32, String> = SimpleCache::new();
    ///
    /// cache.insert(1, String::from("a"));
    /// ```
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        let entry = self.h.insert(
            key,
            ValueWithTimeout {
                value,
                insert_time: Instant::now()
            }
        )?;
        Some(entry.value.clone())
    }

    /// Remove an entry from the cache
    /// ```
    /// use simple_cache_rs::SimpleCache;
    ///
    /// let mut cache: SimpleCache<i32, String> = SimpleCache::new();
    ///
    /// cache.insert(1, String::from("a"));
    /// cache.delete(&1);
    /// ```
    pub fn delete(&mut self, key: &K) -> Option<V> {
        let entry = self.h.remove(key)?; // Returns None, if "fail"
        Some(entry.value.clone())
    }
}

#[cfg(test)]
mod tests {
    use crate::SimpleCache;
    use std::time::Duration;
    use std::thread;

    #[test]
    fn insert_and_get_item() {
        let mut scache: SimpleCache<i32, String> = SimpleCache::new();
        scache.insert(1, String::from("hello"));

        let v = scache.get(&1);

        assert_eq!(Some(String::from("hello")), v)
    }

    #[test]
    fn insert_and_get_item_and_remove() {
        let mut scache: SimpleCache<i32, String> = SimpleCache::new();
        scache.insert(1, String::from("hello"));

        let v = scache.get(&1);

        assert_eq!(Some(String::from("hello")), v);

        scache.delete(&1);

        let no_value = scache.get(&1);

        assert_eq!(None, no_value)
    }

    #[test]
    fn insert_batch_test() {
        let mut scache: SimpleCache<i32, String> = SimpleCache::new();

        scache.insert_batch(vec![(1, String::from("hello")), (2, String::from("world"))]);

        let values = scache.values();

        assert!(values.contains(&&String::from("hello")));
        assert!(values.contains(&&String::from("world")))
    }

    #[test]
    fn get_keys_test() {
        let mut scache: SimpleCache<i32, String> = SimpleCache::new();
        scache.insert(1, String::from("hello"));

        let keys = scache.keys();

        assert_eq!(keys, vec!(1))
    }

    #[test]
    fn get_values_test() {
        let mut scache: SimpleCache<i32, String> = SimpleCache::new();
        scache.insert(1, String::from("hello"));

        let values = scache.values();

        assert_eq!(values, vec!("hello"))
    }


    #[test]
    fn insert_with_timeout() {
        let mut scache: SimpleCache<i32, String> = SimpleCache::with_timeout(Duration::new(1, 0));

        scache.insert(1, String::from("hello"));
        thread::sleep(Duration::new(1, 1));

        let v = scache.get(&1);

        assert_eq!(None, v)
    }
}
