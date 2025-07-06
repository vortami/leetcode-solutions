pub struct Solution;

impl Solution {
    pub fn find_lucky(mut arr: Vec<i32>) -> i32 {
        arr.sort();
        
        let mut out = -1;
        let mut curr = -1;
        let mut freq = 0;

        for &n in &arr {
            if curr == n {
                freq += 1;
            } else {
                if curr == freq && out < curr {
                    out = curr;
                }
                curr = n;
                freq = 1;
            }
        }

        if curr == freq && out < curr {
            out = curr;
        }

        out
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test1() {
        assert_eq!(Solution::find_lucky(vec![2,2,3,4]), 2);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::find_lucky(vec![1,2,2,3,3,3]), 3);
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::find_lucky(vec![2,2,2,3,3]), -1);
    }

    #[test]
    fn test4() {
        assert_eq!(Solution::find_lucky(vec![4,3,2,2,4,1,3,4,3]), 3);
    }
}
