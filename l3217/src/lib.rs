// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

pub struct Solution;

impl Solution {
    pub fn modified_list(mut nums: Vec<i32>, mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        nums.sort();
        let mut tail = &mut head;
        while tail.is_some() {
            let tail_val = tail.as_ref()?.val;
            // while `contains` is negligible in smaller lists, it makes a huge difference to use `binary_search` in bigger lists O(n) vs O(log n)
            if nums.binary_search(&tail_val).is_ok() {
                *tail = tail.take()?.next.take();
            } else {
                tail = &mut tail.as_mut()?.next;
            }
        }
        head
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;


    macro_rules! list {
        ($(,)?) => {
            None
        };
        ($($items:tt),* $(,)?) => {{
            let mut head: ::core::option::Option<::std::boxed::Box<crate::ListNode>> = ::core::option::Option::None;
            let mut tail: &mut ::core::option::Option<::std::boxed::Box<crate::ListNode>> = &mut head;

            $(
                *tail = ::core::option::Option::Some(::std::boxed::Box::new(crate::ListNode { val: $items, next: ::core::option::Option::None }));
                tail = match tail {
                    Some(n) => &mut n.next,
                    None => unsafe { ::core::hint::unreachable_unchecked() },
                };
            )*

            _ = &tail;

            head
        }};
    }

    #[test]
    fn test1() {
        assert_eq!(Solution::modified_list(vec![1,2,3], list![1,2,3,4,5]), list![4,5]);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::modified_list(vec![1], list![1,2,1,2,1,2]), list![2,2,2]);
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::modified_list(vec![5], list![1,2,3,4]), list![1,2,3,4]);
    }
}
