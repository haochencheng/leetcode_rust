struct Solution;

impl Solution {
    pub fn max_vowels(s: String, k: i32) -> i32 {
        let k = k as usize;
        let mut ans = 0;
        let mut count = 0;
        let s = s.as_bytes();

        // Helper function to check if a character is a vowel
        let is_vowel = |&c: &u8| c == b'a' || c == b'e' || c == b'i' || c == b'o' || c == b'u';

        for i in 0..s.len() {
            // Check if the current character is a vowel and update the count
            if is_vowel(&s[i]) {
                count += 1;
            }

            // Once the window reaches size k, adjust the window by removing the leftmost character
            if i >= k {
                if is_vowel(&s[i - k]) {
                    count -= 1;
                }
            }

            // Track the maximum count of vowels within any window
            ans = ans.max(count);

            // Early exit if we reach the maximum possible number of vowels
            if ans == k as i32 {
                return ans;
            }
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
