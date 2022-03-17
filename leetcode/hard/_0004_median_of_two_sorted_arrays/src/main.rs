//! 4. Median of Two Sorted Arrays
//!
//! Given two sorted arrays nums1 and nums2 of size m and n respectively,
//! return the median of the two sorted arrays.
//!
//! The overall run time complexity should be O(log (m+n))

fn main() {
    println!("Hello, world!");
}

struct Solution;

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let mut merged_vec:Vec<i32> = vec![0; nums1.len() + nums2.len()];

        let mut i = 0;
        let mut j = 0;

        while i < nums1.len() && j < nums2.len() {
            if nums1[i] < nums2[j] {
                merged_vec[i + j] = nums1[i];
                i += 1;
            } else {
                if let Some(n1) = nums2.get(j) {
                    merged_vec[i + j] = *n1;
                    j += 1;
                } else {
                    merged_vec[i + j] = nums1[i];
                    i += 1;
                }
            }
        }

        while i < nums1.len() {
            merged_vec[i + j] = nums1[i];
            i += 1;
        }

        while j < nums2.len() {
            merged_vec[i + j] = nums2[j];
            j += 1;
        }

        let mid = merged_vec.len() / 2;

        match merged_vec.len() % 2 {
            0 => (merged_vec[mid-1] as f64 + merged_vec[mid] as f64) / 2.0,
            _ =>  merged_vec[mid] as f64
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(2.0, Solution::find_median_sorted_arrays(vec![1, 3], vec![2]));
    }

    #[test]
    fn test_2() {
        assert_eq!(2.5, Solution::find_median_sorted_arrays(vec![1, 2], vec![3,4]));
    }
}