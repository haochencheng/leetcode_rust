struct Solution {}

impl Solution {
    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        let n = cost.len();
        let mut ans = vec![0; n + 1];
        ans[0] = 0;
        ans[1] = 0;
        for i in 2..=n {
            ans[i] = (ans[i - 1] + cost[i - 1]).min(ans[i - 2] + cost[i - 2]);
        }
        ans[n]
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn maximum_score() {
        let cost = vec![10, 15, 20];
        assert_eq!(Solution::min_cost_climbing_stairs(cost), 15);
        let cost = vec![1, 100, 1, 1, 1, 100, 1, 1, 100, 1];
        assert_eq!(Solution::min_cost_climbing_stairs(cost), 6);
    }
}
