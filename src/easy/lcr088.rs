use core::num;
use std::collections::HashMap;

// 不简单的简单题
pub trait Solution {
    fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32;
}

pub struct MinCostClimbingStairs {}

impl Solution for MinCostClimbingStairs {
    fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        // dp[i]=min(dp[i−1]+cost[i−1],dp[i−2]+cost[i−2])
        let n = cost.len();
        let mut dp = vec![0; n + 1];
        dp[0] = 0;
        dp[1] = 0;
        for i in 2..n + 1 {
            dp[i] = i32::min(dp[i - 1] + cost[i - 1], dp[i - 2] + cost[i - 2])
        }
        return dp[n];
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn path_encryption() {
        let cost = vec![10, 15, 20];
        let result = 15;
        assert_eq!(
            MinCostClimbingStairs::min_cost_climbing_stairs(cost),
            result
        );
    }
}
