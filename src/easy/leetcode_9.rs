// 1593. Split a String Into the Max Number of Unique Substrings

pub trait Solution {
    fn is_palindrome(s: i32) -> bool;
}

pub struct IsPalindrome {}

impl Solution for IsPalindrome {
    fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        }
        if x == 0 {
            return true;
        }
        let s = String::from(x.to_string());
        let i32_chars: Vec<char> = s.chars().collect();
        let count = i32_chars.len();
        let result = true;
        for i in 0..count {
            let j: usize = count - i - 1;
            println!(
                "i======{},h===={},char[i]:{},char[j]:{}",
                i, j, i32_chars[i], i32_chars[j]
            );
            if i32_chars[i] != i32_chars[j] {
                return false;
            }
            if i > j {
                return result;
            };
        }
        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn max_unique_split() {
        // let x = 121;
        // assert!(<IsPalindrome as Solution>::is_palindrome(x));
        // let x = 1214;
        // assert_ne!(<IsPalindrome as Solution>::is_palindrome(x),true);
        let x = 1000021;
        assert_eq!(<IsPalindrome as Solution>::is_palindrome(x), false);
    }
}
