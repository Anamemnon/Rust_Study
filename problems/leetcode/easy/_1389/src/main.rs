//! https://leetcode.com/problems/create-target-array-in-the-given-order/
//! 1389. Create Target Array in the Given Order
//!
//! Given two arrays of integers nums and index. Your task is to create target array under the following rules:
//!
//!     Initially target array is empty.
//!     From left to right read nums[i] and index[i], insert at index index[i] the value nums[i] in target array.
//!     Repeat the previous step until there are no elements to read in nums and index.
//!
//! Return the target array.
//!
//! It is guaranteed that the insertion operations will be valid.
//!

fn main() {
    println!("Hello, world!");
}

struct Solution;

impl Solution {
    pub fn create_target_array(nums: Vec<i32>, index: Vec<i32>) -> Vec<i32> {
        let mut array = Vec::new();

        nums.iter()
            .zip(index.iter())
            .for_each(|(n, i)| array.insert(*i as usize, *n));

        array
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(vec![0,4,1,3,2], Solution::create_target_array(vec![0,1,2,3,4], vec![0,1,2,2,1]));
    }

    #[test]
    fn test_2() {
        assert_eq!(vec![0,1,2,3,4], Solution::create_target_array(vec![1,2,3,4,0], vec![0,1,2,3,0]));
    }
}