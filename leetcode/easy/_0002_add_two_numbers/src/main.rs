//! You are given two non-empty linked lists representing two non-negative integers. The digits are
//! stored in reverse order, and each of their nodes contains a single digit. Add the two numbers and
//! return the sum as a linked list.
//!
//! You may assume the two numbers do not contain any leading zero, except the number 0 itself.
//!
//! Constraints:
//! The number of nodes in each linked list is in the range [1, 100].
//! 0 <= Node.val <= 9
//! It is guaranteed that the list represents a number that does not have leading zeros.

fn main() {
    let l1: Option<Box<ListNode>> =
        Some(Box::new(ListNode {val: 2, next: Some(Box::new(ListNode {val: 4, next: Some(Box::new(ListNode::new(3)))}))}));

    let l2: Option<Box<ListNode>> =
        Some(Box::new(ListNode {val: 5, next: Some(Box::new(ListNode {val: 6, next: Some(Box::new(ListNode::new(4)))}))}));

    // Some(ListNode { val: 7, next: Some(ListNode { val: 0, next: Some(ListNode { val: 8, next: None }) }) })
    println!("{:?}", Solution::add_two_numbers(l1, l2));
}

struct Solution;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val,
        }
    }
}

impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        match (l1, l2) {
            (None, None) => None,
            (Some(n), None) | (None, Some(n)) => Some(n),
            (Some(n1), Some(n2)) => {
                let sum = n1.val + n2.val;

                if sum < 10 {
                    Some(Box::new(ListNode {
                        val: sum,
                        next: Solution::add_two_numbers(n1.next, n2.next),
                    }))
                } else {
                    let carry = Some(Box::new(ListNode::new(1)));

                    Some(Box::new(ListNode {
                        val: sum - 10,
                        next: Solution::add_two_numbers(Solution::add_two_numbers(carry, n1.next), n2.next)
                    }))
                }
            }
        }
    }
}