use std::hash::{DefaultHasher, Hash, Hasher};

const INITIAL_NBUCKETS: usize = 1;

pub struct LinkedHashMap<K, V> {
    buckets: Vec<Vec<(K, V)>>,
}

impl<K, V> LinkedHashMap<K, V>
where
    K: Hash + Eq,
{
    pub fn new() -> Self {
        Self {
            buckets: Vec::new(),
        }
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);

        let bucket = hasher.finish() as usize % self.buckets.len();
        let bucket = &mut self.buckets[bucket];

        for &mut (ref ekey, evalue) in bucket.iter_mut() {
            if ekey == &key {
                return Some(std::mem::replace(evalue, value));
            }
        }

        bucket.push((key, value));

        Some(value)
    }

    pub fn resize(&mut self) {
        let target_size = match self.buckets.len() {
            0 => INITIAL_NBUCKETS,
            n => 2 * n,
        };
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
}
