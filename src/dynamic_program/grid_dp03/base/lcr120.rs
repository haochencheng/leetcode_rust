struct Solution {}

impl Solution {
    pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
        let mut dp: Vec<Vec<i32>> = triangle.iter().map(|row| vec![0; row.len()]).collect();
        dp[0][0] = triangle[0][0];
        for i in 1..triangle.len() {
            for j in 0..triangle[i].len() {
                if j == 0 {
                    dp[i][0] = dp[i - 1][0] + triangle[i][0];
                } else if j == triangle[i].len() - 1 {
                    dp[i][j] = dp[i - 1][j - 1] + triangle[i][j];
                } else {
                    dp[i][j] = dp[i - 1][j - 1].min(dp[i - 1][j]) + triangle[i][j];
                }
            }
        }
        // println!("dp=====:{:#?}", dp);
        *dp[triangle.len() - 1].iter().min().unwrap()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn maximum_score() {
        let triangle = vec![vec![2], vec![3, 4], vec![6, 5, 7], vec![4, 1, 8, 3]];
        assert_eq!(Solution::minimum_total(triangle), 11);
    }
}
