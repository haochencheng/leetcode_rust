struct Solution {}

impl Solution {
    pub fn valid_palindrome(s: String) -> bool {
        let (mut l, mut r) = (0 as usize, s.len() as usize - 1);
        while l < r {
            if s.as_bytes()[l] == s.as_bytes()[r] {
                l += 1;
                r -= 1;
            } else {
                return Solution::is_palindrome(&s, l + 1, r)
                    || Solution::is_palindrome(&s, l, r - 1);
            }
        }
        return true;
    }

    fn is_palindrome(s: &str, b: usize, e: usize) -> bool {
        let mut b = b;
        let mut e = e;
        while b < e {
            if s.as_bytes()[b] != s.as_bytes()[e] {
                return false;
            }
            b += 1;
            e -= 1;
        }
        return true;
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn is_palindrome() {
        assert_eq!(Solution::valid_palindrome("aba".to_string()), true);
        assert_eq!(Solution::valid_palindrome("abca".to_string()), true);
        assert_eq!(Solution::valid_palindrome("abc".to_string()), false);
        assert_eq!(Solution::valid_palindrome("tebbem".to_string()), false);
    }
}
