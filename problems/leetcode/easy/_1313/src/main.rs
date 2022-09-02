//! https://leetcode.com/problems/decompress-run-length-encoded-list/
//! 1313. Decompress Run-Length Encoded List
//!
//! We are given a list nums of integers representing a list compressed with run-length encoding.
//! Consider each adjacent pair of elements [freq, val] = [nums[2*i], nums[2*i+1]] (with i >= 0).
//! For each such pair, there are freq elements with value val concatenated in a sublist.
//! Concatenate all the sublists from left to right to generate the decompressed list.
//! Return the decompressed list.
//!
//! # Example 1:
//!
//! Input: nums = [1,2,3,4]
//! Output: [2,4,4,4]
//! Explanation: The first pair [1,2] means we have freq = 1 and val = 2 so we generate the array [2].
//! The second pair [3,4] means we have freq = 3 and val = 4 so we generate [4,4,4].
//! At the end the concatenation [2] + [4,4,4] is [2,4,4,4].
//!
//! # Example 2:
//!
//! Input: nums = [1,1,2,3]
//! Output: [1,3,3]

fn main() {
    println!("Hello, world!");
}

struct Solution;

impl Solution {
    pub fn decompress_rl_elist(nums: Vec<i32>) -> Vec<i32> {
        let mut len = 0;
        nums.iter().step_by(2).for_each(|freq| len += freq);

        let mut res: Vec<i32> = Vec::with_capacity(len as usize);

        nums.iter().step_by(2)
            .zip(nums.iter().skip(1).step_by(2))
            .for_each(|(freq, val)| res.extend(vec![val; *freq as usize]));

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(vec![2,4,4,4], Solution::decompress_rl_elist(vec![1,2,3,4]))
    }

    #[test]
    fn test_2() {
        assert_eq!(vec![1,3,3], Solution::decompress_rl_elist(vec![1,1,2,3]))
    }
}