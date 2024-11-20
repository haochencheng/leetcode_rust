struct Solution {}

impl Solution {
    pub fn count_house_placements(n: i32) -> i32 {
        const MOD: i64 = 1_000_000_007;
        let mut f1: i64 = 1;
        let mut f2: i64 = 1;
        for _ in 0..n {
            let f3 = (f2 + f1) % MOD;
            f1 = f2;
            f2 = f3;
        }
        (f2 * f2 % MOD) as i32
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn maximum_score() {
        assert_eq!(Solution::count_house_placements(1), 4);
        assert_eq!(Solution::count_house_placements(2), 9);
        assert_eq!(Solution::count_house_placements(2), 25);
    }
}
