pub struct Solution;

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let mut iter1 = nums1.iter().copied().peekable();
        let mut iter2 = nums2.iter().copied().peekable();

        let mut iter = core::iter::from_fn(|| match (iter1.peek(), iter2.peek()) {
            (Some(&a), Some(&b)) => {
                if a < b {
                    iter1.next()
                } else {
                    iter2.next()
                }
            }
            (Some(_), None) => iter1.next(),
            (None, Some(_)) => iter2.next(),
            (None, None) => None,
        });

        let total = nums1.len() + nums2.len();
        if total % 2 == 0 {
            (iter.nth((total - 1) / 2).unwrap() as f64 + iter.next().unwrap() as f64) / 2.0
        } else {
            iter.nth(total.div_euclid(2)).unwrap() as f64
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1, 3], vec![2]),
            2.0
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1, 2], vec![3, 4]),
            2.5
        );
    }
}
