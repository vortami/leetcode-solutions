pub struct Solution;

use core::ops::ControlFlow;

impl Solution {
    pub fn two_edit_words(mut queries: Vec<String>, dictionary: Vec<String>) -> Vec<String> {
        if queries.first().is_none_or(|item| item.len() <= 2) {
            return queries;
        }
        queries
            .extract_if(.., |item| {
                for dict in &dictionary {
                    debug_assert_eq!(item.len(), dict.len());

                    if item
                        .as_bytes()
                        .iter()
                        .zip(dict.as_bytes())
                        .try_fold(0u8, |acc, (l, r)| {
                            if l != r {
                                if acc == 2 {
                                    ControlFlow::Break(())
                                } else {
                                    ControlFlow::Continue(acc + 1)
                                }
                            } else {
                                ControlFlow::Continue(acc)
                            }
                        })
                        .is_continue()
                    {
                        return false;
                    }
                }
                true
            })
            .for_each(drop);
        queries
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // override macro
    macro_rules! vec {
        ($(,)?) => {
            ::std::vec::Vec::<String>::new()
        };
        ($($item:expr),* $(,)?) => {
            ::std::vec![$(
                <::std::string::String as ::core::convert::From<&'static str>>::from($item),
            )*]
        };
    }

    #[test]
    fn test1() {
        let queries = vec!["word", "note", "ants", "wood"];
        let dictionary = vec!["wood", "joke", "moat"];
        assert_eq!(
            Solution::two_edit_words(queries, dictionary),
            vec!["word", "note", "wood"]
        )
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::two_edit_words(vec!["yes"], vec!["not"]), vec![]);
    }
}
