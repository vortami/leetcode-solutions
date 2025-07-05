#![no_std]

use alloc::{boxed::Box, vec};

extern crate alloc;

#[cfg_attr(test, derive(Debug))]
pub struct MyCircularQueue {
    inner: Box<[i32]>,
    len: usize,
    start: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyCircularQueue {
    pub fn new(k: i32) -> Self {
        MyCircularQueue {
            inner: vec![i32::MIN; k as usize].into_boxed_slice(),
            len: 0,
            start: 0,
        }
    }

    pub fn en_queue(&mut self, value: i32) -> bool {
        if self.is_full() {
            false
        } else {
            self.inner[(self.len + self.start) % self.inner.len()] = value;
            self.len += 1;
            true
        }
    }

    pub fn de_queue(&mut self) -> bool {
        if self.is_empty() {
            false
        } else {
            self.start = (self.start + 1) % self.inner.len();
            self.len -= 1;
            true
        }
    }

    pub fn front(&self) -> i32 {
        if self.is_empty() {
            -1
        } else {
            self.inner[self.start]
        }
    }

    pub fn rear(&self) -> i32 {
        if self.is_empty() {
            -1
        } else {
            self.inner[(self.start + (self.len - 1)) % self.inner.len()]
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn is_full(&self) -> bool {
        self.len == self.inner.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    extern crate std;
    use std::dbg;

    #[test]
    fn test1() {
        let mut queue = MyCircularQueue::new(3);
        assert!(queue.en_queue(1));
        assert!(queue.en_queue(2));
        assert!(queue.en_queue(3));
        dbg!(&queue);
        assert!(!queue.en_queue(4));
        assert_eq!(queue.rear(), 3);
        assert!(queue.is_full());
        assert!(queue.de_queue());
        assert!(queue.en_queue(4));
        assert_eq!(queue.rear(), 4);
    }

    #[test]
    fn test2() {
        let mut queue = MyCircularQueue::new(3);
        assert!(queue.en_queue(2));
        assert_eq!(queue.rear(), 2);
        assert_eq!(queue.front(), 2);
        assert!(queue.de_queue());
        assert_eq!(queue.front(), -1);
        assert!(!queue.de_queue());
        assert_eq!(queue.front(), -1);
        assert!(queue.en_queue(4));
        assert!(queue.en_queue(2));
        assert!(queue.en_queue(2));
        assert!(!queue.en_queue(3));
    }
}
