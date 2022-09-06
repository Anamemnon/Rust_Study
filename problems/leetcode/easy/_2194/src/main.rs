//! https://leetcode.com/problems/cells-in-a-range-on-an-excel-sheet/
//! 2194. Cells in a Range on an Excel Sheet
//!
//! A cell (r, c) of an excel sheet is represented as a string "<col><row>" where:
//!
//!     <col> denotes the column number c of the cell. It is represented by alphabetical letters.
//!         For example, the 1st column is denoted by 'A', the 2nd by 'B', the 3rd by 'C', and so on.
//!     <row> is the row number r of the cell. The rth row is represented by the integer r.
//!
//! You are given a string s in the format "<col1><row1>:<col2><row2>", where <col1>
//! represents the column c1, <row1> represents the row r1, <col2> represents the column c2,
//! and <row2> represents the row r2, such that r1 <= r2 and c1 <= c2.
//!
//! Return the list of cells (x, y) such that r1 <= x <= r2 and c1 <= y <= c2.
//! The cells should be represented as strings in the format mentioned above and be sorted
//! in non-decreasing order first by columns and then by rows.
//!
//! # Example
//! Input: s = "K1:L2"
//! Output: ["K1","K2","L1","L2"]

fn main() {
    println!("Hello, world!");
}

struct Solution;

impl Solution {
    pub fn cells_in_range(s: String) -> Vec<String> {
        let chars: Vec<char> = s.chars().collect();
        let mut arr = Vec::new();

        for ch in chars[0]..=chars[3] {
            for i in chars[1]..=chars[4] {
                arr.push(format!("{}{}", ch, i ));
            }
        }

        arr
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(vec!["K1","K2","L1","L2"], Solution::cells_in_range("K1:L2".to_string()))
    }
}