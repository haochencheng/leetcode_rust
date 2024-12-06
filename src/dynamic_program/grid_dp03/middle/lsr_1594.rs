struct Solution {}

impl Solution {
    pub fn max_product_path(grid: Vec<Vec<i32>>) -> i32 {
        const MOD: i64 = 1_000_000_007;
        let m = grid.len();
        let n = grid[0].len();

        let mut dp_max = vec![vec![0i64; n]; m];
        let mut dp_min = vec![vec![0i64; n]; m];

        dp_max[0][0] = grid[0][0] as i64;
        dp_min[0][0] = grid[0][0] as i64;

        // 初始化第一行
        for j in 1..n {
            dp_max[0][j] = dp_max[0][j - 1] * grid[0][j] as i64;
            dp_min[0][j] = dp_max[0][j];
        }

        // 初始化第一列
        for i in 1..m {
            dp_max[i][0] = dp_max[i - 1][0] * grid[i][0] as i64;
            dp_min[i][0] = dp_max[i][0];
        }

        // 填充 DP 表
        for i in 1..m {
            for j in 1..n {
                let curr = grid[i][j] as i64;
                if curr >= 0 {
                    dp_max[i][j] = (dp_max[i - 1][j].max(dp_max[i][j - 1]) * curr) % MOD;
                    dp_min[i][j] = (dp_min[i - 1][j].min(dp_min[i][j - 1]) * curr) % MOD;
                } else {
                    dp_max[i][j] = (dp_min[i - 1][j].min(dp_min[i][j - 1]) * curr) % MOD;
                    dp_min[i][j] = (dp_max[i - 1][j].max(dp_max[i][j - 1]) * curr) % MOD;
                }
            }
        }

        let result = dp_max[m - 1][n - 1];
        if result < 0 {
            -1
        } else {
            (result % MOD) as i32
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn maximum_score() {
        // let grid = vec![vec![-1, -2, -3], vec![-2, -3, -3], vec![-3, -3, -2]];
        // assert_eq!(Solution::max_product_path(grid), -1);
        // let grid = vec![vec![1, -2, 1], vec![1, -2, 1], vec![3, -4, 1]];
        // assert_eq!(Solution::max_product_path(grid), 8);
        let grid = vec![vec![1, 4, 4, 0], vec![-2, 0, 0, 1], vec![1, -1, 1, 1]];
        assert_eq!(Solution::max_product_path(grid), 2);
    }
}
