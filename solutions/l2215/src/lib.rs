pub struct Solution;

impl Solution {
    pub fn find_difference(mut nums1: Vec<i32>, mut nums2: Vec<i32>) -> Vec<Vec<i32>> {
        nums1.sort();
        nums2.sort();

        let mut out = vec![Vec::new(), Vec::new()];

        for &n1 in &nums1 {
            if nums2.binary_search(&n1).is_err() && out[0].binary_search(&n1).is_err() {
                out[0].push(n1);
            }
        }

        for &n2 in &nums2 {
            if nums1.binary_search(&n2).is_err() && out[1].binary_search(&n2).is_err() {
                out[1].push(n2);
            }
        }

        out.iter_mut().for_each(Vec::shrink_to_fit);

        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::find_difference(vec![1, 2, 3], vec![2, 4, 6]), vec![vec![1,3], vec![4, 6]]);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::find_difference(vec![1, 2, 3, 3], vec![1, 1, 2, 2]), vec![vec![3], vec![]]);
    }
}
