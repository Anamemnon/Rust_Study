//! https://leetcode.com/problems/shuffle-string/submissions/
//! 1528. Shuffle String
//!
//! You are given a string s and an integer array indices of the same length.
//! The string s will be shuffled such that the character at the ith position moves to indices[i] in the shuffled string.
//!
//! Return the shuffled string.

fn main() {
    println!("Hello, world!");
}

struct Solution;

impl Solution {
    pub fn restore_string(s: String, indices: Vec<i32>) -> String {
        let mut arr = vec![' '; indices.len()];

        for (i, &v) in indices.iter().enumerate() {
            arr[v as usize] = s.as_bytes()[i] as char;
        }

        arr.iter().collect::<String>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!("leetcode".to_string(), Solution::restore_string("codeleet".to_string(), vec![4,5,6,7,0,2,1,3]));
    }
}