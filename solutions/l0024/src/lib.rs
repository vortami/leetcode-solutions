// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    pub const fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub struct Solution;

impl Solution {
    pub fn swap_pairs(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut ptr = &mut head;
        while ptr.as_ref().is_some_and(|node| node.next.is_some()) {
            let mut left = ptr.take()?;
            let mut right = left.next.take()?;

            let tail = right.next.take();

            right.next = Some(left);
            right.next.as_mut()?.next = tail;

            *ptr = Some(right);
            ptr = &mut ptr.as_mut()?.next.as_mut()?.next;
        }
        head
    }
}

#[cfg(test)]
mod tests {
    use crate::{ListNode, Solution};

    macro_rules! list {
        ($(,)?) => {
            None
        };
        ($($items:tt),* $(,)?) => {{
            let mut head: ::core::option::Option<::std::boxed::Box<crate::ListNode>> = ::core::option::Option::None;
            let mut tail: &mut ::core::option::Option<::std::boxed::Box<crate::ListNode>> = &mut head;

            $(
                *tail = ::core::option::Option::Some(::std::boxed::Box::new(crate::ListNode::new($items)));
                tail = match tail {
                    Some(n) => &mut n.next,
                    None => unsafe { ::core::hint::unreachable_unchecked() },
                };
            )*

            _ = &tail;

            head
        }};
    }

    #[inline]
    fn f(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        Solution::swap_pairs(head)
    }

    #[test]
    fn test1() {
        assert_eq!(f(list![1, 2, 3, 4]), list![2, 1, 4, 3]);
    }

    #[test]
    fn test2() {
        assert_eq!(f(list![]), list![]);
    }

    #[test]
    fn test3() {
        assert_eq!(f(list![1]), list![1]);
    }
}
