fn main() {
    println!("Hello, world!");
}

struct Solution;

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let mut window_size = s.len();
        while window_size > 0 {
            match s.as_bytes()
                .windows(window_size)
                .find(|slice| {
                    let iter = slice.iter();
                    iter.clone().eq(iter.clone().rev()) // cloning iterators is cheap
                }
                ) {
                Some(slice) => return String::from_utf8(slice.to_vec()).unwrap_or("".to_string()),
                None => window_size -= 1, // if no slice window of this size is a palindrome, try the next smallest window size
            }
        }
        "".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_a() {
        assert_eq!("a".to_string(), Solution::longest_palindrome("a".to_string()))
    }

    #[test]
    fn test_aaa() {
        assert_eq!("aaa".to_string(), Solution::longest_palindrome("aaa".to_string()))
    }
}