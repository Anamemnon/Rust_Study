//! 3. Longest Substring Without Repeating Characters
//!
//! Given a string s, find the length of the longest substring without repeating characters.

use std::collections::HashMap;

fn main() {
    println!("{}", Solution::length_of_longest_substring("".to_string()));
}

struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut table = HashMap::new();

        let mut start_index: i32 = -1;
        let mut max_length: i32 = 0;

        for (end_index, c) in s.chars().enumerate() {
            if let Some(old_end_index) = table.insert(c, end_index) {
                start_index = std::cmp::max(start_index, old_end_index as i32);
            }

            max_length = std::cmp::max(max_length, end_index as i32 - start_index as i32);
        }

        max_length
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_abcabcbb() {
        assert_eq!(3, Solution::length_of_longest_substring("abcabcbb".to_string()))
    }

    #[test]
    fn test_bbbbb() {
        assert_eq!(1, Solution::length_of_longest_substring("bbbbb".to_string()))
    }

    #[test]
    fn test_pwwkew() {
        assert_eq!(3, Solution::length_of_longest_substring("pwwkew".to_string()))
    }

    #[test]
    fn test_p() {
        assert_eq!(1, Solution::length_of_longest_substring("p".to_string()))
    }

}