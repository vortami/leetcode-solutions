use std::collections::VecDeque;

pub struct Solution;

impl Solution {
    pub fn count_students(students: Vec<i32>, sandwiches: Vec<i32>) -> i32 {
        let mut noops = 0;
        let mut students = VecDeque::from(students);
        let mut sandwiches = VecDeque::from(sandwiches);
        while !students.is_empty() && noops < students.len() {
            if students[0] == sandwiches[0] {
                noops = 0;
                students.pop_front();
                sandwiches.pop_front();
            } else {
                noops += 1;
                students.rotate_left(1);
            }
        }
        students.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::count_students(vec![1, 1, 0, 0], vec![1, 1, 0, 0]),
            0
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::count_students(vec![1, 1, 1, 0, 0, 1], vec![1, 0, 0, 0, 1, 1]),
            3
        );
    }
}
