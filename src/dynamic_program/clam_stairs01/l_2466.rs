struct Solution {}

impl Solution {
    pub fn count_good_strings(low: i32, high: i32, zero: i32, one: i32) -> i32 {
        // sum(dp[low..high])
        // 每次可以选zero或者one
        // dp[0]=1
        // 选dp[i]=dp[i-zero]+dp[i-one]
        let mut dp = vec![0; high as usize + 1];
        let mod_val = 1_000_000_007;
        dp[0] = 1; // Base case: 1 way to form a string of length 0.

        // Fill dp array using previous values based on the lengths zero and one
        for i in 1..=high as usize {
            if i >= zero as usize {
                dp[i] = (dp[i] + dp[i - zero as usize]) % mod_val;
            }
            if i >= one as usize {
                dp[i] = (dp[i] + dp[i - one as usize]) % mod_val;
            }
        }

        // Sum dp values from low to high
        let mut ans = 0;
        for i in low as usize..=high as usize {
            ans = (ans + dp[i]) % mod_val;
        }

        ans
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn maximum_score() {
        let low = 3;
        let high = 3;
        let zero = 1;
        let one = 1;
        assert_eq!(Solution::count_good_strings(low, high, zero, one), 8);
    }
}
