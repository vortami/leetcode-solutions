pub struct Solution;

impl Solution {
    pub fn count_subarrays(nums: Vec<i32>) -> i32 {
        let mut count = 0;
        for window in nums.windows(3) {
            assert_eq!(window.len(), 3);
            if (window[0] + window[2]) * 2 == window[1] {
                count += 1;
            }
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::count_subarrays(vec![1, 2, 1, 4, 1]), 1);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::count_subarrays(vec![1, 1, 1]), 0);
    }
}
