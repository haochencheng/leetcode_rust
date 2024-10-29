// 674. 最长连续递增序列

pub trait Solution {
    fn valid_palindrome(s: String) -> bool;

    fn valid_palindrome_sub(s: &[u8]) -> bool;
}

pub struct FindLengthOfLcis {}

impl Solution for FindLengthOfLcis {
    fn valid_palindrome(s: String) -> bool {
        if s.is_empty() {
            return false;
        }
        let (mut l, mut r) = (0, s.len() - 1);
        let s_bytes = s.as_bytes();
        while l < r {
            if s_bytes[l] == s_bytes[r] {
                l += 1;
                r -= 1;
            } else {
                return Self::valid_palindrome_sub(&s_bytes[l + 1..=r])
                    || Self::valid_palindrome_sub(&s_bytes[l..=r - 1]);
            }
        }
        return true;
    }

    fn valid_palindrome_sub(s: &[u8]) -> bool {
        for i in 0..s.len() {
            if s[i] != s[s.len() - i - 1] {
                return false;
            }
        }
        return true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn judge_circle() {
        let s: String = "aba".to_string();
        assert_eq!(FindLengthOfLcis::valid_palindrome(s), true);
        let s: String = "abca".to_string();
        assert_eq!(FindLengthOfLcis::valid_palindrome(s), true);
        let s: String = "cbbcc".to_string();
        assert_eq!(FindLengthOfLcis::valid_palindrome(s), true);
    }
}
