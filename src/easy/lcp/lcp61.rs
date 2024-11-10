use std::i32;

struct Solution {}

impl Solution {
    pub fn temperature_trend(temperature_a: Vec<i32>, temperature_b: Vec<i32>) -> i32 {
        if temperature_a.is_empty() || temperature_b.is_empty() {
            return 0;
        }
        let mut t_a = vec![0; temperature_a.len() - 1];
        let mut t_b: Vec<i32> = vec![0; temperature_b.len() - 1];
        let mut max = i32::MIN;
        let mut next = 0;
        for i in 1..temperature_a.len() {
            if temperature_a[i] > temperature_a[i - 1] {
                t_a[i - 1] = 1;
            } else if temperature_a[i] == temperature_a[i - 1] {
                t_a[i - 1] = 0;
            } else {
                t_a[i - 1] = -1;
            }
            if temperature_b[i] > temperature_b[i - 1] {
                t_b[i - 1] = 1;
            } else if temperature_b[i] == temperature_b[i - 1] {
                t_b[i - 1] = 0;
            } else {
                t_b[i - 1] = -1;
            }
            if t_a[i - 1] == t_b[i - 1] {
                next += 1;
            } else {
                max = max.max(next);
                next = 0;
            }
        }

        max.max(next)
    }
}

#[cfg(test)]
mod tests {

    use std::vec;

    use super::*;

    #[test]
    fn min_num_booths() {
        let temperature_a = vec![21, 18, 18, 18, 31];
        let temperature_b = vec![34, 32, 16, 16, 17];
        assert_eq!(Solution::temperature_trend(temperature_a, temperature_b), 2);
    }
}
