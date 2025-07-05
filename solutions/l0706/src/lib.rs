use core::hash::Hasher;
use std::hash::DefaultHasher;

pub struct MyHashMap {
    /// (hash, (key, value))
    inner: Vec<(u64, Vec<(i32, i32)>)>,
}

fn hash(n: i32) -> u64 {
    let mut hasher = DefaultHasher::new();
    hasher.write_i32(n);
    hasher.finish()
}

impl MyHashMap {
    pub const fn new() -> Self {
        Self { inner: Vec::new() }
    }

    pub fn put(&mut self, key: i32, value: i32) {
        let hash = hash(key);
        match self.inner.binary_search_by_key(&hash, |&(hash, _)| hash) {
            Ok(idx) => {
                let (_, inner) = &mut self.inner[idx];
                match inner.binary_search_by_key(&key, |&(key, _)| key) {
                    Ok(idx) => inner[idx].1 = value,
                    Err(idx) => inner.insert(idx, (key, value)),
                }
            }
            Err(idx) => self.inner.insert(idx, (hash, vec![(key, value)])),
        }
    }

    pub fn get(&self, key: i32) -> i32 {
        let Ok(idx) = self
            .inner
            .binary_search_by_key(&hash(key), |&(hash, _)| hash)
        else {
            return -1;
        };
        let (_, inner) = &self.inner[idx];
        match inner.binary_search_by_key(&key, |&(key, _)| key) {
            Ok(idx) => inner[idx].1,
            Err(_) => -1,
        }
    }

    pub fn remove(&mut self, key: i32) {
        if let Ok(idx) = self.inner.binary_search_by_key(&hash(key), |&(hash, _)| hash) {
            let inner = &mut self.inner[idx].1;
            if let Ok(idx) = inner.binary_search_by_key(&key, |&(key, _)| key) {
                inner.remove(idx);
            }
            if inner.is_empty() {
                self.inner.remove(idx);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut obj = MyHashMap::new();
        obj.put(1, 1);
        obj.put(2, 2);
        assert_eq!(obj.get(1), 1);
        assert_eq!(obj.get(3), -1);
        obj.put(2, 1);
        assert_eq!(obj.get(2), 1);
        obj.remove(2);
        assert_eq!(obj.get(2), -1);
    }
}
