pub struct Solution;

impl Solution {
    pub fn find_the_distance_value(arr1: Vec<i32>, arr2: Vec<i32>, d: i32) -> i32 {
        let mut count = 0;
        'arr1: for &item1 in &arr1 {
            for &item2 in &arr2 {
                if (item1 - item2).abs() <= d {
                    continue 'arr1;
                }
            }
            count += 1;
        }
        count
    }
}

#[cfg(test)]
mod tests {
    macro_rules! define_test {
        ($name:ident($arr1:expr, $arr2: expr, $d: expr) -> $rv:expr) => {
            #[test]
            fn $name() {
                ::std::assert_eq!($crate::Solution::find_the_distance_value(<[i32]>::to_vec(&$arr1), <[i32]>::to_vec(&$arr2), $d), $rv);
            }
        };
    }

    define_test!(test1([4,5,8], [10,9,1,8], 2) -> 2);
    define_test!(test2([1,4,2,3], [-4,-3,6,10,20,30], 3) -> 2);
    define_test!(test3([2,1,100,3], [-5,-2,10,-3,7], 6) -> 1);
}
