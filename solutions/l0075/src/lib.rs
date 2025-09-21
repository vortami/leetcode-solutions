// slice::sort is disallowed
#![forbid(clippy::disallowed_methods)]

pub struct Solution;

impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let mut idx = 0;
        while idx < nums.len() - 1 {
            if nums[idx] > nums[idx + 1] {
                nums.swap(idx, idx + 1);
                idx = idx.saturating_sub(1);
            } else {
                idx += 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    macro_rules! test {
        ($testname:ident: [$($in:expr),* $(,)?] -> [$($out:expr),* $(,)?]) => {
            #[test]
            fn $testname() {
                let mut v = <[i32]>::to_vec(&[$($in),*]);
                $crate::Solution::sort_colors(&mut v);
                ::std::assert_eq!(v, <[i32]>::to_vec(&[$($out),*]));
            }
        };
    }

    test!(test1: [2, 0, 2, 1, 1, 0] -> [0, 0, 1, 1, 2, 2]);
    test!(test2: [2, 0, 1] -> [0, 1, 2]);
}
