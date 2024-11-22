use std::i32;

struct Solution {}

impl Solution {
    pub fn min_falling_path_sum(matrix: Vec<Vec<i32>>) -> i32 {
        let n = matrix.len();
        let mut dp: Vec<Vec<i32>> = vec![vec![0; n]; n];
        for i in 0..n {
            for j in 0..n {
                if i == 0 {
                    dp[i][j] = matrix[i][j];
                } else if j == 0 {
                    dp[i][0] = dp[i - 1][0].min(dp[i - 1][1]) + matrix[i][j];
                } else if j == n - 1 {
                    dp[i][j] = (dp[i - 1][j]).min(dp[i - 1][j - 1]) + matrix[i][j];
                } else {
                    dp[i][j] =
                        (dp[i - 1][j + 1]).min(dp[i - 1][j]).min(dp[i - 1][j - 1]) + matrix[i][j];
                }
            }
        }
        println!("dp=====:{:#?}", dp);
        *dp[n - 1].iter().min().unwrap()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn maximum_score() {
        let matrix = vec![vec![2, 1, 3], vec![6, 5, 4], vec![7, 8, 9]];
        assert_eq!(Solution::min_falling_path_sum(matrix), 13);

        let matrix = vec![
            vec![100, -42, -46, -41],
            vec![31, 97, 10, -10],
            vec![-58, -51, 82, 89],
            vec![51, 81, 69, -51],
        ];
        assert_eq!(Solution::min_falling_path_sum(matrix), -36);

        let matrix = vec![vec![82, 81], vec![69, 33]];
        assert_eq!(Solution::min_falling_path_sum(matrix), 114);
    }
}
