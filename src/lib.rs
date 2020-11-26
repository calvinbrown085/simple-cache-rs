use std::collections::HashMap;
use std::hash::Hash;
use std::time::Duration;

struct ValueWithTimeout<V: Copy> {
    value: V,
    timeout: Option<Duration>,
}

pub struct SimpleCache<K, V> {
    h: HashMap<K, V>,
    timeout: Option<Duration>
}


impl<K: Eq + Hash + Clone, V: Clone + Copy> SimpleCache<K, ValueWithTimeout<V>> {

    pub fn new() -> SimpleCache<K, ValueWithTimeout<V>> {
        SimpleCache{ h: HashMap::new(), timeout: None }
    }

    pub fn with_timeout(timeout: Duration) -> SimpleCache<K, ValueWithTimeout<V>> {
        SimpleCache{ h: HashMap::new(), timeout: Some(timeout) }
    }

    pub fn get(&mut self, key: &K) -> Option<V> {
        self.h.get(key).and_then(|vwt| {
            match vwt.timeout {
                Some(t) => {
                    if self.timeout >= Some(t) {
                        self.delete(key);
                        None
                    }
                    else {
                        Some(vwt.value)
                    }
                },
                None => Some(vwt.value)
            }
        })
    }

    pub fn keys(&mut self) -> Vec<&K> {
        self.h.keys().collect::<Vec<&K>>()
    }

    pub fn values(&mut self) -> Vec<&V> {
        self.h.values().map(|v| &v.value).collect::<Vec<&V>>()
    }

    pub fn insert_batch(&mut self, items: Vec<(K, V)>) {
        for item in items {
            self.h.insert(item.0, ValueWithTimeout { value: item.1, timeout: self.timeout }).map(|v| v.value);
        }
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        self.h.insert(key, ValueWithTimeout { value, timeout: self.timeout }).map(|v| v.value)
    }

    pub fn delete(&mut self, key: &K) -> Option<V> {
        self.h.remove(key).map(|v| v.value)
    }
}




#[cfg(test)]
mod tests {
    use crate::SimpleCache;

    #[test]
    fn insert_and_get_item() {
        let mut scache: SimpleCache<i32, String> = SimpleCache::new(None);
        scache.insert(1, String::from("hello"));

        let v = scache.get(&1);

        assert_eq!(Some(&String::from("hello")), v)
    }

    #[test]
    fn insert_and_get_item_and_remove() {
        let mut scache: SimpleCache<i32, String> = SimpleCache::new(None);
        scache.insert(1, String::from("hello"));

        let v = scache.get(&1);

        assert_eq!(Some(&String::from("hello")), v);

        scache.delete(&1);

        let no_value = scache.get(&1);

        assert_eq!(None, no_value)
    }

    #[test]
    fn insert_batch_test() {
        let mut scache: SimpleCache<i32, String> = SimpleCache::new(None);

        scache.insert_batch(vec!((1, String::from("hello")),(2, String::from("world"))));

        let values = scache.values();

        assert_eq!(values, vec!("hello", "world"))
    }


    #[test]
    fn get_keys_test() {
        let mut scache: SimpleCache<i32, String> = SimpleCache::new(None);
        scache.insert(1, String::from("hello"));

        let keys = scache.keys();

        assert_eq!(keys, vec!(&1))
    }

    #[test]
    fn get_values_test() {
        let mut scache: SimpleCache<i32, String> = SimpleCache::new(None);
        scache.insert(1, String::from("hello"));

        let values = scache.values();

        assert_eq!(values, vec!("hello"))
    }
}
