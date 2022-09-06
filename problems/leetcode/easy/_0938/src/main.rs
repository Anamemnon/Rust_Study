//! https://leetcode.com/problems/range-sum-of-bst/submissions/
//! 938. Range Sum of BST
//!
//! Given the root node of a binary search tree and two integers low and high,
//! return the sum of values of all nodes with a value in the inclusive range [low, high].
//! # Example
//! Input: root = [10,5,15,3,7,null,18], low = 7, high = 15
//! Output: 32
//! Explanation: Nodes 7, 10, and 15 are in the range [7, 15]. 7 + 10 + 15 = 32.

fn main() {
    println!("Hello, world!");
}


// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub val: i32,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    TreeNode {
      val,
      left: None,
      right: None
    }
  }
}

use std::rc::Rc;
use std::cell::RefCell;

struct Solution;

impl Solution {
    pub fn range_sum_bst(root: Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> i32 {
        let mut sum = 0;

        match root {
            Some(v) => {
                let node = v.borrow();
                let val = node.val;
                if val >= low && val <= high {
                    sum += val;
                };

                sum += Self::range_sum_bst(node.left.clone(), low, high);
                sum += Self::range_sum_bst(node.right.clone(), low, high);

                sum
            }
            None => 0
        }
    }
}