struct Solution {}

impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let y = m as usize;
        let x = n as usize;
        if y == 0 {
            return 0;
        }
        let mut dp = vec![vec![0; x]; y];
        // dp[i + 1][j + 1] = max(dp[j + 1][i], dp[j][i + 1]) + frame[j][i];
        for i in 0..x {
            for j in 0..y {
                if i == 0 || j == 0 {
                    dp[j][i] = 1;
                } else {
                    dp[j][i] = dp[j - 1][i] + dp[j][i - 1];
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
        assert_eq!(Solution::unique_paths(3, 7), 28);
    }
}
