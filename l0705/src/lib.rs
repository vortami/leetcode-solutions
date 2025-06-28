use core::hash::Hasher;
use std::hash::DefaultHasher;

pub struct MyHashSet {
    inner: Vec<(u64, i32)>,
}

fn hash(n: i32) -> u64 {
    let mut hasher = DefaultHasher::new();
    hasher.write_i32(n);
    hasher.finish()
}

impl MyHashSet {
    pub const fn new() -> Self {
        Self { inner: Vec::new() }
    }

    pub fn add(&mut self, key: i32) {
        let hash = hash(key);
        match self.inner.binary_search_by_key(&&hash, |(hash, _)| hash) {
            Ok(_) => { /* already in set */ }
            Err(idx) => self.inner.insert(idx, (hash, key)),
        }
    }

    pub fn remove(&mut self, key: i32) {
        let hash = hash(key);
        match self.inner.binary_search_by_key(&&hash, |(hash, _)| hash) {
            Ok(idx) => {
                self.inner.remove(idx);
            }
            Err(_) => { /* not in set */ }
        }
    }

    pub fn contains(&self, key: i32) -> bool {
        let hash = hash(key);
        self.inner
            .binary_search_by_key(&&hash, |(hash, _)| hash)
            .is_ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut obj = MyHashSet::new();
        obj.add(1);
        obj.add(2);
        assert!(obj.contains(1));
        assert!(!obj.contains(3));
        obj.add(2);
        assert!(obj.contains(2));
        obj.remove(2);
        assert!(!obj.contains(2));
    }
}
