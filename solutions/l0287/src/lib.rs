pub struct Solution;

impl Solution {
    pub fn find_duplicate(nums: Vec<i32>) -> i32 {
        let mut seen = vec![false; nums.len()];

        for num in nums {
            if seen[num as usize] {
                return num;
            } else {
                seen[num as usize] = true;
            }
        }

        unreachable!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::find_duplicate(vec![1,3,4,2,2]), 2);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::find_duplicate(vec![3,1,3,4,2]), 3);
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::find_duplicate(vec![3,3,3,3,3]), 3);
    }
}
