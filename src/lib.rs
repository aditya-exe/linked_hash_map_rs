use std::{
    borrow::Borrow,
    hash::{DefaultHasher, Hash, Hasher},
};

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

    pub fn contains_key<Q>(&self, key: &Q) -> bool
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        self.get(key.borrow()).is_some()
    }

    pub fn get<Q>(&self, key: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        let bucket_idx = self.get_bucket(key);
        self.buckets[bucket_idx]
            .iter()
            .find(|&(ekey, _)| ekey.borrow() == key)
            .map(|&(_, ref evalue)| evalue)
    }

    pub fn remove<Q>(&mut self, key: &Q) -> Option<V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        let bucket_idx = self.get_bucket(key);
        let item_idx = self.buckets[bucket_idx]
            .iter()
            .position(|&(ref ekey, _)| ekey.borrow() == key)?;

        self.items -= 1;

        Some(self.buckets[bucket_idx].swap_remove(item_idx).1)
    }

    pub fn len(&self) -> usize {
        self.items
    }

    pub fn is_empty(&self) -> bool {
        self.items == 0
    }

    fn resize(&mut self) {
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

    fn get_bucket<Q>(&self, key: &Q) -> usize
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);

        (hasher.finish() % self.buckets.len() as u64) as usize
    }
}

pub struct Iter<'a, K: 'a, V: 'a> {
    map: &'a LinkedHashMap<K, V>,
    bucket: usize,
    at: usize,
}

impl<'a, K, V> Iterator for Iter<'a, K, V> {
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.map.buckets.get(self.at) {
                Some(bucket) => match bucket.get(self.at) {
                    Some(&(ref k, ref v)) => {
                        self.at += 1;
                        break Some((k, v));
                    }
                    None => {
                        self.bucket += 1;
                        self.at = 0;

                        continue;
                    }
                },
                None => break None,
            }
        }
    }
}

impl<'a, K, V> IntoIterator for &'a LinkedHashMap<K, V> {
    type Item = (&'a K, &'a V);
    type IntoIter = Iter<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        Iter {
            map: self,
            bucket: 0,
            at: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert_test() {
        let mut map = LinkedHashMap::new();
        map.insert("abc", 123);

        assert_eq!(map.len(), 1);
    }

    #[test]
    fn get_test() {
        let mut map = LinkedHashMap::new();
        map.insert("abc", 123);

        assert_eq!(map.get(&"abc"), Some(&123));
    }

    #[test]
    fn remove_test() {
        let mut map = LinkedHashMap::new();
        map.insert("abc", 123);

        assert_eq!(map.len(), 1);

        map.remove(&"abc");

        assert_eq!(map.len(), 0);
    }
}
