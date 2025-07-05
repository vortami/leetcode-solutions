/// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub const fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub struct Solution;

impl Solution {
    pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
        let mut as_vec = Vec::new();

        {
            let mut tail = &head;
            while let Some(_t) = tail {
                as_vec.push(&**_t);
                tail = &_t.next;
            }
        }

        let half = as_vec.len() / 2;
        let (left, mut right) = as_vec.split_at_mut(half);

        if left.len() != right.len() {
            assert!(left.len() + 1 == right.len());
            right = &mut right[1..];
        }

        left.reverse();

        left.iter().zip(right).all(|(l, r)| l.val == r.val)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn from_arr<const N: usize>(v: [i32; N]) -> Option<Box<ListNode>> {
        let mut head = None;
        let mut tail = &mut head;
        for n in v {
            *tail = Some(Box::new(ListNode::new(n)));
            tail = &mut tail.as_mut()?.next;
        }
        head
    }

    #[test]
    fn test1() {
        assert!(Solution::is_palindrome(from_arr([1, 2, 2, 1])));
        assert!(!Solution::is_palindrome(from_arr([1, 2])));
        assert!(Solution::is_palindrome(from_arr([1, 2, 1])));
    }
}
