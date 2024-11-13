use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn max_vowels(s: String, k: i32) -> i32 {
        let mut o_set = HashSet::new();
        let o_arr = vec!['a', 'e', 'i', 'o', 'u'];
        for value in o_arr {
            o_set.insert(value);
        }
        let mut ans = 0;
        for i in 0..s.len() {
            let mut count = 0;
            let window = &s[i..i + k as usize];
            for x in window.chars() {
                if o_set.contains(&x) {
                    count += 1;
                }
            }
            ans = ans.max(count);
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn max_unique_split() {
        assert_eq!(Solution::max_vowels("abciiidef".to_string(), 3), 3);
        assert_eq!(Solution::max_vowels("aeiou".to_string(), 2), 2);
    }
}
