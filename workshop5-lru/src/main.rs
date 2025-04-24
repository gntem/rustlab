use std::collections::HashMap;

struct LRUCache {
    capacity: usize,
    cache: HashMap<i32, i32>,
    order: Vec<i32>,
}

impl LRUCache {
    fn new(capacity: i32) -> Self {
        LRUCache {
            capacity: capacity as usize,
            cache: HashMap::new(),
            order: Vec::new(),
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        if let Some(&value) = self.cache.get(&key) {
            self.order.retain(|&k| k != key);
            self.order.push(key);
            value
        } else {
            -1
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        if self.cache.len() == self.capacity {
            if let Some(&lru_key) = self.order.first() {
                self.cache.remove(&lru_key);
                self.order.remove(0);
            }
        }
        self.cache.insert(key, value);
        self.order.retain(|&k| k != key);
        self.order.push(key);
    }
}


fn main() {
    let mut c = LRUCache::new(2);
    c.put(1, 1);
    c.put(2, 2);
    println!("Cache after adding 1 and 2: {:?}", c.cache);
    assert_eq!(c.get(1), 1);
    c.put(3, 3);
    println!("Cache after adding 3: {:?}", c.cache);
    assert_eq!(c.get(2), -1);
    c.put(4, 4);
    println!("Cache after adding 4: {:?}", c.cache);
}