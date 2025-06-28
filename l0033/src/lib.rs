pub struct Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        if nums.is_empty() {
            return -1;
        }

        if nums[0] > target {
            nums.iter()
                .enumerate()
                .rev()
                .take_while(|&(_, &n)|n >= target)
                .find(|&(_, &n)| n == target)
                .map_or(-1, |(idx, _)| idx as i32)
        } else {
            nums.iter()
                .enumerate()
                .take_while(|&(_, &n)| n <= target)
                .find(|&(_, &n)| n == target)
                .map_or(-1, |(idx, _)| idx as i32)
        }
    }
}

#[cfg(test)]
mod tests {
    macro_rules! define_test {
        ($fname:ident($($arg:expr),*) -> $rv:expr) => {
            #[test]
            fn $fname() {
                ::std::assert_eq!(
                    $crate::Solution::search($($arg,)*),
                    $rv
                );
            }
        };
    }

    define_test!(test1(vec![4,5,6,7,0,1,2], 0) -> 4);
    define_test!(test2(vec![4,5,6,7,0,1,2], 3) -> -1);
    define_test!(test3(vec![1], 0) -> -1);
}
