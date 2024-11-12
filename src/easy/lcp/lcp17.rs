struct Solution {}

impl Solution {
    pub fn calculate(s: String) -> i32 {
        let (x, y) = s.chars().fold((1, 0), |(x, y), c| match c {
            'A' => (2 * x + y, y),
            'B' => (x, 2 * y + x),
            _ => (x, y),
        });
        x + y
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn maximum_score() {
        assert_eq!(Solution::calculate("AB".to_string()), 4);
    }
}
