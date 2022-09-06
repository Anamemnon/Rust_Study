//! https://leetcode.com/problems/number-of-steps-to-reduce-a-number-to-zero/
//! 1342. Number of Steps to Reduce a Number to Zero
//!
//! Given an integer num, return the number of steps to reduce it to zero.
//!
//! In one step, if the current number is even, you have to divide it by 2,
//! otherwise, you have to subtract 1 from it.
//!
//! This task can be solved with bit counts:
//!     for each 0 in bit representation we need 1 operation ( / 2)
//!     for each 1 in bit representation (except one first) we need 2 operations (-1 and / 2)
//!
//! For example:
//! 14 = 1 1 1 0
//! ops=1 2 2 1 = 6

fn main() {
    println!("Hello, world!");
}

struct Solution;

impl Solution {
    pub fn number_of_steps(num: i32) -> i32 {
        if num > 1 {
            (num.count_zeros() - num.leading_zeros() + 2 * num.count_ones() - 1) as i32
        } else {
            num
        }
    }
}