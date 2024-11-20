struct Solution {}

impl Solution {
    pub fn count_texts(pressed_keys: String) -> i32 {
        let mod_val = 1_000_000_007;
        let chars: Vec<char> = pressed_keys.chars().collect();
        let n = chars.len();
        let mut dp = vec![0; n + 1];
        dp[0] = 1; // Base case: one way to decode an empty string.

        for i in 1..=n {
            dp[i] = dp[i - 1]; // Single press of the current key.

            // Check up to 3 or 4 repeated presses depending on the key.
            let max_presses = if "79".contains(chars[i - 1]) { 4 } else { 3 };

            for j in 2..=max_presses {
                if i >= j && chars[i - 1] == chars[i - j] {
                    dp[i] = (dp[i] + dp[i - j]) % mod_val;
                } else {
                    break;
                }
                println!("dp======{:#?}", dp);
            }
        }

        dp[n]
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn maximum_score() {
        let pressedKeys = "22233".to_string();
        assert_eq!(Solution::count_texts(pressedKeys), 8);
    }
}
