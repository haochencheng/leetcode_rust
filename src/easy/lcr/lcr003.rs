struct Solution {}

impl Solution {
    pub fn count_bits(n: i32) -> Vec<i32> {
        let mut ans = vec![];
        for i in 0..=n {
            ans.push(Self::count_bit(i));
        }
        ans
    }

    fn count_bit(mut n: i32) -> i32 {
        let mut bits = 0;
        while n > 0 {
            n = n & (n - 1);
            bits += 1;
        }
        bits
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn is_palindrome() {
        assert_eq!(Solution::count_bits(2), vec![0, 1, 1]);
    }
}
