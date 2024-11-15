struct Solution {}

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let arr = Vec::new();
        arr[0] = 0;
        arr[1] = 1;
        for i in 2..n {
            arr[i] = arr[i - 2] + arr[i - 1];
        }
        arr[n - 1]
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn maximum_score() {
        let relation = vec![4, 2, 1];
        assert_eq!(Solution::climb_stairs(relation), 4);
    }
}
