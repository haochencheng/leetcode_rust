use std::{collections::HashMap, sync::Arc};

struct Solution;

impl Solution {
    pub fn max_score(card_points: Vec<i32>, k: i32) -> i32 {
        let n = card_points.len();
        let k = k as usize;
        let mut sum: i32 = card_points[0..n - k].iter().sum();
        let mut ans = sum;
        for i in n - k..n {
            sum += card_points[i];
            sum -= card_points[i - (n - k)];
            ans = sum.min(ans);
        }
        card_points.iter().sum::<i32>() - ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn max_unique_split() {
        let nums = vec![1, 2, 3, 4, 5, 6, 1];
        let k = 3;
        let ans = 12;
        assert_eq!(Solution::max_score(nums, k), ans);
    }
}
