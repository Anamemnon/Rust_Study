//! https://leetcode.com/problems/add-two-integers/
//! 2235. Add Two Integers
//! Given two integers num1 and num2, return the sum of the two integers.

fn main() {
    println!("Hello, world!");
}

struct Solution;

impl Solution {
    pub fn sum(num1: i32, num2: i32) -> i32 {
        num1 + num2
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(5, Solution::sum(2, 3))
    }
}
