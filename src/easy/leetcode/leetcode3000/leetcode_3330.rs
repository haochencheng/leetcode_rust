struct Solution {}

impl Solution {
    pub fn possible_string_count(word: String) -> i32 {
        let mut pre = b'0';
        let mut sum: i32 = 0;
        let mut cur = 1;
        for (_i, &byte) in word.as_bytes().iter().enumerate() {
            if byte == pre {
                cur += 1;
            } else {
                if cur != 0 {
                    sum += cur - 1;
                }
                cur = 1;
            }
            pre = byte;
        }
        if cur != 1 {
            sum += cur - 1;
        }
        sum + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_balanced() {
        assert_eq!(Solution::possible_string_count("abbcccc".to_string()), 5);
        assert_eq!(Solution::possible_string_count("aaaa".to_string()), 4);
    }
}
