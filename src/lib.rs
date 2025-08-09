use std::hash::{DefaultHasher, Hash, Hasher};

const INITIAL_NBUCKETS: usize = 1;

pub struct LinkedHashMap<K, V> {
    buckets: Vec<Vec<(K, V)>>,
    items: usize,
}

impl<K, V> LinkedHashMap<K, V>
where
    K: Hash + Eq,
{
    pub fn new() -> Self {
        Self {
            buckets: Vec::new(),
            items: 0,
        }
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        if self.buckets.is_empty() || self.items > 3 * self.buckets.len() / 4 {
            self.resize();
        }

        let bucket_idx = self.get_bucket(&key);
        let bucket = &mut self.buckets[bucket_idx];

        for &mut (ref ekey, ref mut evalue) in bucket.iter_mut() {
            if ekey == &key {
                return Some(std::mem::replace(evalue, value));
            }
        }

        bucket.push((key, value));
        self.items += 1;

        None
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        let bucket_idx = self.get_bucket(key);
        self.buckets[bucket_idx]
            .iter()
            .find(|&(ekey, _)| ekey == key)
            .map(|&(_, ref evalue)| evalue)
    }

    pub fn resize(&mut self) {
        let target_size = match self.buckets.len() {
            0 => INITIAL_NBUCKETS,
            n => 2 * n,
        };

        let mut new_buckets = Vec::with_capacity(target_size);
        new_buckets.extend((0..target_size).map(|_| Vec::new()));

        for (key, value) in self.buckets.iter_mut().flat_map(|bucket| bucket.drain(..)) {
            let mut hasher = DefaultHasher::new();
            key.hash(&mut hasher);

            let bucket = (hasher.finish() % new_buckets.len() as u64) as usize;
            new_buckets[bucket].push((key, value));
        }

        let _ = std::mem::replace(&mut self.buckets, new_buckets);
    }

    fn get_bucket(&self, key: &K) -> usize {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);

        (hasher.finish() % self.buckets.len() as u64) as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert_test() {
        let mut map = LinkedHashMap::new();
        map.insert("abc", 123);
    }

    #[test]
    fn get_test() {
        let mut map = LinkedHashMap::new();
        map.insert("abc", 123);

        assert_eq!(map.get(&"abc"), Some(&123));
    }
}
