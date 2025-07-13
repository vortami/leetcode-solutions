pub struct Solution;

impl Solution {
    pub fn smallest_range_i(nums: Vec<i32>, k: i32) -> i32 {
        let mut min = i32::MAX;
        let mut max = i32::MIN;
        for &n in &nums {
            min = min.min(n);
            max = max.max(n);
        }
        let diff = max - min;
        if diff > k*2 {
            diff - k*2
        } else {
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::smallest_range_i(vec![1], 0), 0);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::smallest_range_i(vec![0,10], 2), 6);
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::smallest_range_i(vec![1,3,6], 3), 0);
    }

    #[test]
    fn test4() {
        assert_eq!(Solution::smallest_range_i(vec![2,7,2], 1), 3);
    }
}
