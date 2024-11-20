struct Solution {}

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let n = n as usize;
        let mut arr = vec![0; n + 1];
        arr[0] = 1;
        arr[1] = 1;
        for i in 2..=n {
            arr[i] = arr[i - 2] + arr[i - 1];
        }
        arr[n]
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn maximum_score() {
        assert_eq!(Solution::climb_stairs(2), 2);
        assert_eq!(Solution::climb_stairs(3), 3);
    }
}
