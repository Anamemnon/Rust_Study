//! https://leetcode.com/problems/maximum-number-of-words-found-in-sentences/
//! 2114. Maximum Number of Words Found in Sentences
//! A sentence is a list of words that are separated by a single space with no leading or trailing spaces.
//! You are given an array of strings sentences, where each sentences[i] represents a single sentence.
//! Return the maximum number of words that appear in a single sentence.

fn main() {
    println!("Hello, world!");
}

struct Solution;

impl Solution {
    pub fn most_words_found(sentences: Vec<String>) -> i32 {
        let mut max = 0;

        sentences.iter().for_each(|s| {
            let local_max = s.split_whitespace().count();
            if local_max > max {max = local_max}
        });

        max as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(6, Solution::most_words_found(vec!["alice and bob love leetcode".to_string(),
                                                      "i think so too".to_string(),
                                                      "this is great thanks very much".to_string()]))
    }
}

