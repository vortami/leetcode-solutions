pub struct Solution;

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let mut chars_iter = s.chars();

        loop {
            let front = loop {
                match chars_iter.next() {
                    Some(c) if c.is_ascii_alphanumeric() => break c.to_ascii_lowercase(),
                    Some(_) => continue,
                    None => return true,
                }
            };
            let back = loop {
                match chars_iter.next_back() {
                    Some(c) if c.is_ascii_alphanumeric() => break c.to_ascii_lowercase(),
                    Some(_) => continue,
                    None => return true,
                }
            };
            if front != back {
                return false;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::is_palindrome("A man, a plan, a canal: Panama".to_owned()),
            true
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::is_palindrome("0P".to_owned()),
            false
        );
    }
}
