//! https://leetcode.com/problems/decode-xored-array/
//! 1720. Decode XORed Array
//!
//! Return the original array arr. It can be proved that the answer exists and is unique.
//!
//! # Example 1
//!
//! Input: encoded = [1,2,3], first = 1
//! Output: [1,0,2,1]
//! Explanation: If arr = [1,0,2,1], then first = 1 and encoded = [1 XOR 0, 0 XOR 2, 2 XOR 1] = [1,2,3]

fn main() {
    println!("Hello, world!");
}

struct Solution;

impl Solution {
    pub fn decode(encoded: Vec<i32>, first: i32) -> Vec<i32> {
        let mut res = Vec::with_capacity(encoded.len() + 1);
        res.push(first);

        for i in 0..encoded.len() {
            res.push(encoded[i] ^ res[i]);
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(vec![1,0,2,1], Solution::decode(vec![1,2,3], 1))
    }

    #[test]
    fn test_2() {
        assert_eq!(vec![4,2,0,7,4], Solution::decode(vec![6,2,7,3], 4))
    }
}
