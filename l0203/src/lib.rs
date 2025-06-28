/// Definition for singly-linked list.
#[derive(PartialEq, Eq)]
#[cfg_attr(test, derive(Debug))]
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
    pub fn remove_elements(mut head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        let mut tail = &mut head;
        while tail.is_some() {
            if tail.as_ref().map(|node| node.val) == Some(val) {
                *tail = tail.as_mut().and_then(|node| node.next.take());
            } else {
                tail = match tail {
                    Some(node) => &mut node.next,
                    None => unreachable!(),
                }
            }
        }

        head
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    
    macro_rules! list {
        () => {
            None
        };
        ($($elt:expr),* $(,)?) => {{
            let mut head = None;
            // using underscore so compiler doesn't complain on last item
            let mut _tail = &mut head;
            $(
                *_tail = Some(Box::new($crate::ListNode::new($elt)));
                _tail = &mut unsafe { _tail.as_mut().unwrap_unchecked() }.next;
            )*
            head
        }};
    }

    #[test]
    fn test1() {
        assert_eq!(Solution::remove_elements(list![1, 2, 6, 3, 4, 5, 6], 6), list![1,2,3,4,5]);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::remove_elements(list![], 1), list![]);
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::remove_elements(list![7,7,7,7], 7), list![]);
    }
}
