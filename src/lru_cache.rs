use std::{
    collections::{HashMap, VecDeque},
    hash::Hash,
    rc::Rc,
};

#[derive(Debug)]
struct LRUCache<K: Eq + Hash, V> {
    capacity: usize,
    elements: HashMap<Rc<K>, V>,
    order: VecDeque<Rc<K>>,
}

impl<K: Eq + Hash, V> LRUCache<K, V> {
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity,
            elements: HashMap::with_capacity(capacity),
            order: VecDeque::with_capacity(capacity),
        }
    }

    pub fn get(&mut self, key: &K) -> Option<&V> {
        self.elements.get(key)
    }

    pub fn put(&mut self, key: K, value: V) {
        if self.elements.contains_key(&key) {
            let idx = self.order.iter().position(|k| k.as_ref() == &key).unwrap();
            let elem = self.order.remove(idx).unwrap();
            self.order.push_front(elem);
        } else {
            if self.order.len() >= self.capacity {
                let to_remove = self.order.pop_back().unwrap();
                self.elements.remove(&to_remove).unwrap();
            }
            let k = Rc::new(key);
            self.order.push_front(Rc::clone(&k));
            self.elements.insert(Rc::clone(&k), value);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_cache_returns_none() {
        let mut cache: LRUCache<&'static str, i32> = LRUCache::new(10);
        assert_eq!(cache.get(&"Some key"), None);
    }

    #[test]
    fn cached_key_is_returned() {
        let mut cache = LRUCache::new(10);
        cache.put("key1", 1234);
        assert_eq!(cache.get(&"key1"), Some(&1234));
    }

    #[test]
    fn lru_entry_is_evictetd_when_the_cache_is_full() {
        let mut cache = LRUCache::new(3);
        cache.put("key1", 1);
        cache.put("key2", 2);
        cache.put("key3", 3);
        cache.put("key4", 4);
        assert_eq!(cache.get(&"key1"), None);
    }

    #[test]
    fn lru_entry_is_evictetd_when_the_cache_is_full2() {
        let mut cache = LRUCache::new(4);
        cache.put("key1", 1);
        cache.put("key2", 2);
        cache.put("key3", 3);
        cache.put("key4", 4);
        cache.put("key5", 5);
        cache.put("key1", 7);
        assert_eq!(cache.get(&"key2"), None);
    }
}
