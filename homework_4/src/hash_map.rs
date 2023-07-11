use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

struct HashMap<K, V> {
    buckets: Vec<Vec<(K, V)>>,
    size: usize,
}

impl<K, V> HashMap<K, V>
where
    K: Eq + Hash,
{
    fn new() -> Self {
        let initial_capacity = 16;
        HashMap {
            buckets: vec![Vec::new(); initial_capacity],
            size: 0,
        }
    }

    fn hash(&self, key: &K) -> u64 {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        hasher.finish()
    }

    fn insert(&mut self, key: K, value: V) {
        let hash = self.hash(&key);
        let bucket_index = (hash % self.buckets.len() as u64) as usize;

        for &(ref existing_key, _) in &self.buckets[bucket_index] {
            if existing_key == &key {
                return; // Key already exists, update value
            }
        }

        self.buckets[bucket_index].push((key, value));
        self.size += 1;
    }

    fn get(&self, key: &K) -> Option<&V> {
        let hash = self.hash(key);
        let bucket_index = (hash % self.buckets.len() as u64) as usize;

        for &(ref existing_key, ref value) in &self.buckets[bucket_index] {
            if existing_key == key {
                return Some(value);
            }
        }

        None
    }

    fn remove(&mut self, key: &K) -> Option<V> {
        let hash = self.hash(key);
        let bucket_index = (hash % self.buckets.len() as u64) as usize;

        let bucket = &mut self.buckets[bucket_index];
        let position = bucket.iter().position(|&(ref existing_key, _)| existing_key == key);

        if let Some(index) = position {
            let (_, value) = bucket.remove(index);
            self.size -= 1;
            Some(value)
        } else {
            None
        }
    }
}
fn main() {
    let mut map = HashMap::new();
    map.insert("key1", "value1");
    map.insert("key2", "value2");

    println!("{:?}", map.get(&"key1")); // Выводит: Some("value1")
    println!("{:?}", map.get(&"key3")); // Выводит: None

    println!("{:?}", map.remove(&"key1")); // Выводит: Some("value1")
    println!("{:?}", map.remove(&"key3")); // Выводит: None

    println!("{:?}", map.get(&"key1")); // Выводит: None
}
