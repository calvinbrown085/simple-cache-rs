use std::collections::HashMap;
use std::hash::Hash;
use std::time::Duration;
use std::time::Instant;

#[derive(Debug, Clone)]
struct ValueWithTimeout<V> {
    value: V,
    insert_time: Instant,
    timeout: Option<Duration>,

}

#[derive(Debug, Clone)]
pub struct SimpleCache<K, V> {
    h: HashMap<K, ValueWithTimeout<V>>,
    timeout: Option<Duration>,
}

impl<K: Eq + Hash + Clone, V: Clone> SimpleCache<K, V> {
    pub fn new() -> SimpleCache<K, V> {
        SimpleCache {
            h: HashMap::new(),
            timeout: None,
        }
    }


    pub fn with_timeout(timeout: Duration) -> SimpleCache<K, V> {
        SimpleCache {
            h: HashMap::new(),
            timeout: Some(timeout),
        }
    }

    pub fn get(&mut self, key: &K) -> Option<V> {
        self.h.get(key).and_then(|v| {
            match v.timeout {
                Some(timeout) => {
                    if v.insert_time.elapsed() >= timeout {
                        self.clone().delete(key);
                        None
                    } else {
                        Some(v.value.clone())
                    }
                },
                None => Some(v.value.clone())
            }
        })
    }

    pub fn keys(&self) -> Vec<K> {
        self.h.keys().map(|k| k.clone()).collect::<Vec<K>>()
    }

    pub fn values(&self) -> Vec<V> {
        self.h.values().map(|vwt| vwt.value.clone()).collect::<Vec<V>>()
    }

    pub fn insert_batch(&mut self, items: Vec<(K, V)>) {
        for item in items {
            self.h.insert(
                item.0,
                ValueWithTimeout {
                    value: item.1,
                    insert_time: Instant::now(),
                    timeout: self.timeout,
                },
            );
        }
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        self.h.insert(
            key,
            ValueWithTimeout {
                value,
                insert_time: Instant::now(),
                timeout: self.timeout
            },
        ).map(|vwt| vwt.value.clone())
    }

    pub fn delete(&mut self, key: &K) -> Option<V> {
        self.h.remove(key).map(|vwt| vwt.value)
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
