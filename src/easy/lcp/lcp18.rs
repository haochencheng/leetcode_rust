use std::{i32, usize};

struct Solution {}

impl Solution {
    pub fn breakfast_number(staple: Vec<i32>, drinks: Vec<i32>, x: i32) -> i32 {
        if staple.is_empty() {
            return 0;
        }
        let mut sort_staple = staple;
        sort_staple.sort_unstable();
        let mut sort_drinks = drinks;
        sort_drinks.sort_unstable();

        let mut ans = 0;
        let (mut i, mut j) = (0, (sort_drinks.len() - 1) as i32);
        while i < sort_staple.len() && j >= 0 {
            if sort_staple[i] + sort_drinks[j as usize] <= x {
                i += 1;
                ans += j + 1;
                ans %= 1000000007
            } else {
                j -= 1;
            }
        }
        ans as i32
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn maximum_score() {
        let staple = vec![10, 20, 5];
        let drinks = vec![2, 5, 5];
        let x = 15;
        assert_eq!(Solution::breakfast_number(staple, drinks, x), 6);
        let staple = vec![2, 1, 1];
        let drinks = vec![1, 5, 8, 9];
        let x = 9;
        assert_eq!(Solution::breakfast_number(staple, drinks, x), 8);
    }
}
