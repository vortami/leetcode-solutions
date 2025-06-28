pub struct LRUCache {
    /// [((key, val), time)]
    inner: Vec<((i32, i32), i32)>,
    time: i32,
}

impl LRUCache {
    pub fn new(capacity: i32) -> Self {
        Self {
            inner: Vec::with_capacity(capacity as usize),
            time: 0,
        }
    }

    pub fn get(&mut self, key: i32) -> i32 {
        self.inner
            .iter_mut()
            .find_map(|((k, val), time)| {
                if *k == key {
                    self.time += 1;
                    *time = self.time;
                    Some(*val)
                } else {
                    None
                }
            })
            .unwrap_or(-1)
    }

    pub fn put(&mut self, key: i32, value: i32) {
        self.time += 1;
        if let Some(((_, v), t)) = self.inner.iter_mut().find(|((k, _), _)| *k == key) {
            *v = value;
            *t = self.time;
        } else if self.inner.len() == self.inner.capacity() {
            let Some(((k, v), t)) = self.inner.iter_mut().min_by_key(|(_, time)| *time) else {
                unreachable!();
            };
            *k = key;
            *v = value;
            *t = self.time;
        } else {
            self.inner.push(((key, value), self.time))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut obj = LRUCache::new(2);
        obj.put(1, 1);
        obj.put(2, 2);
        assert_eq!(obj.get(1), 1);
        obj.put(3, 3);
        assert_eq!(obj.get(2), -1);
        obj.put(4, 4);
        assert_eq!(obj.get(1), -1);
        assert_eq!(obj.get(3), 3);
        assert_eq!(obj.get(4), 4);
    }
}
