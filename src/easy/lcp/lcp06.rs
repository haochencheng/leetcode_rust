struct Solution {}

impl Solution {
    pub fn min_count(coins: Vec<i32>) -> i32 {
        let mut ans = 0;
        for mut ele in coins {
            while ele > 0 {
                ans += 1;
                ele -= 2;
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn maximum_score() {
        let relation = vec![4, 2, 1];
        assert_eq!(Solution::min_count(relation), 4);
    }
}
