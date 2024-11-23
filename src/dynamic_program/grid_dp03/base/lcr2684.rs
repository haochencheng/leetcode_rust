use std::i32;

struct Solution {}

impl Solution {
    pub fn max_moves(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();

        // 初始化 DP 表，dp[i][j] 表示从 (i, j) 出发能移动的最大次数
        let mut dp = vec![vec![0; n]; m];

        // 从第一列的所有单元格开始
        for i in 0..m {
            dp[i][0] = 1; // 初始化第一列
        }

        // 动态规划计算每个单元格的最大移动次数
        for j in 0..n - 1 {
            // 遍历每一列（除了最后一列）
            for i in 0..m {
                if dp[i][j] != 0 {
                    // 检查能否向右上移动
                    if i > 0 && grid[i][j] < grid[i - 1][j + 1] {
                        dp[i - 1][j + 1] = dp[i - 1][j + 1].max(dp[i][j] + 1);
                    }
                    // 检查能否向右移动
                    if grid[i][j] < grid[i][j + 1] {
                        dp[i][j + 1] = dp[i][j + 1].max(dp[i][j] + 1);
                    }
                    // 检查能否向右下移动
                    if i < m - 1 && grid[i][j] < grid[i + 1][j + 1] {
                        dp[i + 1][j + 1] = dp[i + 1][j + 1].max(dp[i][j] + 1);
                    }
                }
            }
        }

        // 找到所有 dp[i][n-1] 中的最大值 - 1
        dp.into_iter().flatten().max().unwrap_or(1) - 1
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn maximum_score() {
        let grid: Vec<Vec<i32>> = vec![
            vec![2, 4, 3, 5],
            vec![5, 4, 9, 3],
            vec![3, 4, 2, 11],
            vec![10, 9, 13, 15],
        ];
        assert_eq!(Solution::max_moves(grid), 3);
        let grid: Vec<Vec<i32>> = vec![vec![3, 2, 4], vec![2, 1, 9], vec![1, 1, 7]];
        assert_eq!(Solution::max_moves(grid), 0);
    }
}
