pub struct Solution;

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let mut chars_iter = s.chars();

        loop {
            let Some(front) = chars_iter.find(|c| c.is_ascii_alphanumeric()) else {
                return true;
            };
            let Some(back) = chars_iter.rfind(|c| c.is_alphanumeric()) else {
                return true;
            };
            if !front.eq_ignore_ascii_case(&back) {
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
        assert_eq!(Solution::is_palindrome("0P".to_owned()), false);
    }
}
