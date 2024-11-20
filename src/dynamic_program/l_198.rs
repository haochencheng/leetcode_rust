struct Solution {}

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        // dp[i]= max(dp[i-1],dp[i-2])
        let n = nums.len();
        if n == 1 {
            return nums[0];
        }
        let mut dp = vec![0; n + 1];
        dp[0] = nums[0];
        dp[1] = nums[0].max(nums[1]);
        for i in 2..n {
            dp[i] = (nums[i] + dp[i - 2]).max(dp[i - 1]);
        }
        dp[n - 1]
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn maximum_score() {
        let nums = vec![1, 2, 3, 1];
        assert_eq!(Solution::rob(nums), 4);
        let nums = vec![2, 7, 9, 3, 1];
        assert_eq!(Solution::rob(nums), 12);
    }
}
