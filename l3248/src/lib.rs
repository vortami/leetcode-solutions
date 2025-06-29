pub struct Solution;

impl Solution {
    pub fn final_position_of_snake(n: i32, commands: Vec<String>) -> i32 {
        let mut x = 0;
        let mut y = 0;
        for command in &commands {
            match &**command {
                "UP" => y -= 1,
                "DOWN" => y += 1,
                "LEFT" => x -= 1,
                "RIGHT" => x += 1,
                _ => unreachable!("invalid move")
            }
        }
        x + y * n
    }
}

#[cfg(test)]
mod tests {
    macro_rules! define_testcase {
        ($fname:ident($n:expr, [$($moves:expr),* $(,)?]) -> $rv:expr) => {
            #[test]
            fn $fname() {
                ::std::assert_eq!(
                    $crate::Solution::final_position_of_snake(
                        $n,
                        ::core::iter::IntoIterator::into_iter(
                            [$($moves,)*]
                        )
                            .map(ToOwned::to_owned)
                            .collect::<::std::vec::Vec<::std::string::String>>()
                    ),
                    $rv,
                );
            }
        };
    }

    define_testcase!(test1(2, ["RIGHT", "DOWN"]) -> 3);
    define_testcase!(test2(3, ["DOWN", "RIGHT", "UP"]) -> 1);
}
