pub struct Solution;

impl Solution {
    pub fn clear_digits(s: String) -> String {
        assert!(s.is_ascii());
        let mut bytes = s.into_bytes();

        let mut idx = 0;
        let mut removed = 0;
        while idx < bytes.len() - removed {
            if bytes[idx].is_ascii_digit() {
                if idx == 0 {
                    bytes.rotate_left(1);
                    removed += 1;
                } else {
                    assert!(!bytes[idx - 1].is_ascii_digit());
                    bytes[idx - 1..].rotate_left(2);
                    removed += 2;
                    idx -= 1;
                }
            } else {
                idx += 1;
            }
        }

        bytes.truncate(bytes.len() - removed);

        String::from_utf8(bytes).unwrap_or_else(|_| unreachable!())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::clear_digits(String::from("abc")),
            String::from("abc")
        );
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::clear_digits(String::from("cb34")), String::new());
    }

    #[test]
    fn test3() {
        assert_eq!(
            Solution::clear_digits(String::from("acwb34")),
            String::from("ac")
        );
    }
}
