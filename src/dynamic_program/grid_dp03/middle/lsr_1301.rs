struct Solution {}

impl Solution {
    pub fn paths_with_max_score(board: Vec<String>) -> Vec<i32> {
        const MOD: i64 = 1_000_000_007;
        let grid: Vec<Vec<char>> = board.into_iter().map(|row| row.chars().collect()).collect();

        let m = grid.len();
        let n = grid[0].len();

        let mut dp = vec![vec![0; n]; m];
        dp[0][0] = 0;
        // 第一列
        for i in 0..m {
            dp[i][0] = grid[i][0] as i32;
        }
        // 第一行
        for i in 0..n {
            dp[0][i] = grid[0][i] as i32;
        }

        // 状态转移方程
        // dp[i,j]=max(dp[i-1][j],dp[i][j-1])
        for i in 1..m {
            for j in 1..n {
                if grid[i][j] == 'x' {
                    dp[i][j] = 0;
                } else {
                    dp[i][j] = dp[i - 1][j].max(dp[i][j - 1]) + grid[i][j] as i32;
                }
            }
        }
        dp
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn maximum_score() {
        let grid = vec![vec![1, 4, 4, 0], vec![-2, 0, 0, 1], vec![1, -1, 1, 1]];
        assert_eq!(Solution::paths_with_max_score(grid), 2);
    }
}
