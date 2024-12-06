use std::i32;

struct Solution {}

impl Solution {
    pub fn min_falling_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut dp = vec![vec![0; n]; m];
        for i in 0..n {
            dp[0][i] = grid[0][i];
        }
        for i in 1..m {
            for j in 0..n {
                let mut ans = i32::MAX;
                for k in 0..n {
                    if j == k {
                        continue;
                    }
                    ans = ans.min(dp[i - 1][k] + grid[i][j])
                }
                dp[i][j] = ans;
            }
        }
        let mut res = i32::MAX;
        // println!("res===={:#?}", dp);
        for i in 0..m {
            res = res.min(dp[m - 1][i]);
        }

        res
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn maximum_score() {
        let grid = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        assert_eq!(Solution::min_falling_path_sum(grid), 13);
    }
}
