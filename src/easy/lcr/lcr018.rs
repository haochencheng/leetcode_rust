struct Solution {}

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let s = s
            .chars()
            .filter(|x| x.is_alphanumeric()) // 剔除无效字符
            .map(|x| x.to_lowercase().nth(0).unwrap()) // 转小写
            .collect::<String>();

        return s == s.chars().rev().collect::<String>(); // 和反转后的字符串比较
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn is_palindrome() {
        assert_eq!(Solution::is_palindrome("aba".to_string()), true);
        assert_eq!(Solution::is_palindrome("acbca".to_string()), true);
        assert_eq!(Solution::is_palindrome("abccba".to_string()), false);
    }
}
