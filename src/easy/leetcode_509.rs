// 509. 斐波那契数

pub trait Solution {
 fn fib(n: i32) -> i32 ;
}


pub struct Fib {}

impl Solution for Fib {

    fn fib(n: i32) -> i32 {
        if n <= 1 {
            return n;
        }
        let mut dp = vec![0;(n+1) as usize];
        dp[1]=1;

        for i in 2..=n as usize {
            dp[i]=dp[i-1]+dp[i-2];
        }
        
        dp[n as usize]

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn max_unique_split() {
        let t = 2;
        assert_eq!(Fib::fib(t),1);
        let t = 3;
        assert_eq!(Fib::fib(t),2);
        let t = 4;
        assert_eq!(Fib::fib(t),3);
        
    }
}
