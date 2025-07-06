pub struct FindSumPairs {
    nums1: Vec<(i32, i32)>,
    nums2: Vec<i32>,
    freq_map: Vec<(i32, i32)>,
}

impl FindSumPairs {
    pub fn new(mut nums1: Vec<i32>, nums2: Vec<i32>) -> Self {
        Self {
            nums1: {
                nums1.sort();
                let mut n1 = Vec::<(i32, i32)>::new();
                for n in nums1 {
                    match n1.binary_search_by_key(&n, |&(n, _)| n) {
                        Ok(idx) => n1[idx].1 += 1,
                        Err(idx) => n1.insert(idx, (n, 1)),
                    }
                }
                n1.shrink_to_fit();
                n1
            },
            freq_map: nums2.iter().fold(Vec::new(), |mut acc, &n| {
                match acc.binary_search_by_key(&n, |&(n, _)| n) {
                    Ok(idx) => acc[idx].1 += 1,
                    Err(idx) => acc.insert(idx, (n, 1)),
                }
                acc
            }),
            nums2,
        }
    }

    pub fn add(&mut self, index: i32, val: i32) {
        let n = &mut self.nums2[index as usize];
        match self.freq_map.binary_search_by_key(n, |&(n, _)| n) {
            Ok(idx) => self.freq_map[idx].1 -= 1,
            Err(_) => unreachable!(),
        }
        *n += val;
        match self.freq_map.binary_search_by_key(n, |&(n, _)| n) {
            Ok(idx) => self.freq_map[idx].1 += 1,
            Err(idx) => self.freq_map.insert(idx, (*n, 1)),
        }
    }

    pub fn count(&mut self, tot: i32) -> i32 {
        let mut out = 0;
        for &(n, nf) in &self.nums1 {
            let complement = tot - n;
            if let Ok(idx) = self.freq_map.binary_search_by_key(&complement, |&(n, _)| n) {
                out += nf * self.freq_map[idx].1;                
            }
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut obj = FindSumPairs::new(vec![1, 1, 2, 2, 2, 3], vec![1, 4, 5, 2, 5, 4]);
        assert_eq!(obj.count(7), 8);
        obj.add(3, 2);
        assert_eq!(obj.count(8), 2);
        assert_eq!(obj.count(4), 1);
        obj.add(0, 1);
        obj.add(1, 1);
        assert_eq!(obj.count(7), 11);
    }
}
