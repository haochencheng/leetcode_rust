struct Solution {}

impl Solution {
    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        let y = obstacle_grid.len();
        if y == 0 {
            return 0;
        }
        let x = obstacle_grid[0].len();
        let mut grid = obstacle_grid;
        let mut dp = vec![vec![0; x]; y];
        // dp[i + 1][j + 1] = max(dp[j + 1][i], dp[j][i + 1]) + frame[j][i];
        let mut fill = false;
        for i in 0..x {
            if grid[0][i] == 1 {
                fill = true;
            } else {
                if fill {
                    grid[0][i] = 1;
                }
            }
        }
        fill = false;
        for i in 0..y {
            if grid[i][0] == 1 {
                fill = true;
            } else {
                if fill {
                    grid[i][0] = 1;
                }
            }
        }
        // println!("grid===={:#?}", grid);
        for i in 0..x {
            for j in 0..y {
                if i == 0 || j == 0 {
                    if grid[j][i] == 1 {
                        dp[j][i] = 0;
                    } else {
                        dp[j][i] = 1;
                    }
                } else {
                    if grid[j][i] == 1 {
                        dp[j][i] = 0;
                    } else {
                        dp[j][i] = dp[j - 1][i] + dp[j][i - 1];
                    }
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
        let obstacleGrid = vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]];
        assert_eq!(Solution::unique_paths_with_obstacles(obstacleGrid), 2);
        let obstacleGrid = vec![vec![0, 1], vec![0, 0]];
        assert_eq!(Solution::unique_paths_with_obstacles(obstacleGrid), 1);
    }
}
