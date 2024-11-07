use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }

        let mut char_count = HashMap::new();
        for ch in s.chars() {
            *char_count.entry(ch).or_insert(0) += 1;
        }
        // 比较次数
        for ch in t.chars() {
            let count = char_count.get(&ch);
            if count.is_none() {
                return false;
            }
            if let Some(c) = count {
                if *c == 0 {
                    return false;
                }
                char_count.insert(ch, c - 1);
            }
        }
        let mut s_index = 0;
        let s_chars: Vec<char> = s.chars().collect();
        // 比较顺序
        for ch in t.chars() {
            if s_index < s_chars.len() && ch == s_chars[s_index] {
                s_index += 1;
            }
        }
        return s_index != s_chars.len();
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn moving_average() {
        let (s, t) = ("anagram".to_string(), "nagaram".to_string());
        assert!(Solution::is_anagram(s, t));
    }
}
