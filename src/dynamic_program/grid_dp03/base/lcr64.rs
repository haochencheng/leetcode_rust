struct Solution {}

impl Solution {
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        let y = grid.len();
        let x = grid[0].len();
        if y == 0 {
            return 0;
        }
        if y == 0 {
            return 0;
        }
        let mut dp = vec![vec![0; x + 1]; y + 1];
        // dp[i + 1][j + 1] = max(dp[j + 1][i], dp[j][i + 1]) + frame[j][i];
        dp[0][0] = grid[0][0];
        for i in 0..x {
            for j in 0..y {
                if i == 0 && j == 0 {
                    continue;
                }
                if i == 0 {
                    dp[j][0] = dp[j - 1][0] + grid[j][0];
                } else if j == 0 {
                    dp[0][i] = dp[0][i - 1] + grid[0][i];
                } else {
                    dp[j][i] = dp[j - 1][i].min(dp[j][i - 1]) + grid[j][i];
                }
            }
        }
        // println!("dp===={:#?}", dp);
        dp[y - 1][x - 1]
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn maximum_score() {
        let grid = vec![vec![1, 3, 1], vec![1, 5, 1], vec![4, 2, 1]];
        assert_eq!(Solution::min_path_sum(grid), 7);
    }
}
