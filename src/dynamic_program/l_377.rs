struct Solution {}

impl Solution {
    pub fn combination_sum4(nums: Vec<i32>, target: i32) -> i32 {
        // f[n]=(f[0](0..nums)) + (f[1](0..nums-1)) + (f[n-1](1))
        let mut dp = vec![0; target as usize + 1];
        dp[0] = 1;
        for i in 1..=target as usize {
            for &x in &nums {
                if x as usize <= i {
                    dp[i] += dp[i - x as usize];
                    println!("dp===={:#?}", dp);
                }
            }
        }
        dp[target as usize]
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn maximum_score() {
        let nums = vec![1, 2, 3];
        let target = 4;
        assert_eq!(Solution::combination_sum4(nums, target), 7);
    }
}
