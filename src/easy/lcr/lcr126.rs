use std::i32;
//
pub trait Solution {
    fn fib(num: i32) -> i32;
}

pub struct Fib {}

impl Solution for Fib {
    fn fib(n: i32) -> i32 {
        if n == 0 {
            return 0;
        }
        if n == 1 {
            return 1;
        }
        let mut ans = vec![0; (n + 1) as usize];
        ans[0] = 0;
        ans[1] = 1;
        // println!("ans len {}", ans.len());
        for i in 2..(n + 1) as usize {
            // println!("n====={}", n);
            ans[i] = (ans[i - 1] + ans[i - 2]) % 1000000007
        }
        // println!("ans :{:#?}", ans);
        return ans[(n) as usize];
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn judge_circle() {
        assert_eq!(Fib::fib(2), 1);
        assert_eq!(Fib::fib(3), 2);
        assert_eq!(Fib::fib(45), 134903163);
    }
}
