struct Solution {}

impl Solution {
    pub fn jewellery_value(frame: Vec<Vec<i32>>) -> i32 {
        let y = frame.len();
        let x = frame[0].len();
        if y == 0 {
            return 0;
        }
        let mut dp = vec![vec![0; x + 1]; y + 1];
        // dp[i + 1][j + 1] = max(dp[j + 1][i], dp[j][i + 1]) + frame[j][i];
        for i in 0..x {
            for j in 0..y {
                dp[j + 1][i + 1] = dp[j + 1][i].max(dp[j][i + 1]) + frame[j][i];
            }
        }
        println!("dp===={:#?}", dp);
        dp[y][x]
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn maximum_score() {
        let frame = vec![vec![1, 3, 1], vec![1, 5, 1], vec![4, 2, 1]];
        assert_eq!(Solution::jewellery_value(frame), 12);
        let frame = vec![vec![0, 1], vec![1, 0]];
        assert_eq!(Solution::jewellery_value(frame), 1);
    }
}
