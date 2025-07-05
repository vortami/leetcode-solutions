pub struct Solution;

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        assert!(digits.len() <= 4);
        if digits.is_empty() {
            return Vec::new();
        }

        let combinations: Vec<&str> = digits
            .chars()
            .map(|digit| match digit {
                '1' => panic!("1 does not have any assigned letters"),
                '2' => "abc",
                '3' => "def",
                '4' => "ghi",
                '5' => "jkl",
                '6' => "mno",
                '7' => "pqrs",
                '8' => "tuv",
                '9' => "wxyz",
                '0' => panic!("0 does not have any assigned letters"),
                _ => panic!("not a char"),
            })
            .collect();

        let mut idxs = vec![0; combinations.len()];
        let mut out = Vec::with_capacity(combinations.iter().map(|s| s.len()).sum());
        loop {
            out.push(
                combinations
                    .iter()
                    .zip(&idxs)
                    .map(|(s, i)| match s.chars().nth(*i) {
                        Some(c) => c,
                        None => unreachable!(),
                    })
                    .collect(),
            );
            
            // increase idx
            for (idx, n) in idxs.iter_mut().enumerate().rev() {
                let max = combinations[idx].len();
                *n += 1;
                if *n < max {
                    break;
                } else {
                    *n = 0;
                }
            }
            if idxs.iter().all(|&n| n == 0) {
                break;
            }
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let solutions = ["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"];

        let out = Solution::letter_combinations(String::from("23"));

        assert_eq!(out.len(), solutions.len());
        for s in &solutions {
            assert!(solutions.contains(s));
        }
    }

    #[test]
    fn test2() {
        let solutions = ["a", "b", "c"];

        let out = Solution::letter_combinations(String::from("2"));

        assert_eq!(out.len(), solutions.len());
        for s in &solutions {
            assert!(solutions.contains(s));
        }
    }
}
