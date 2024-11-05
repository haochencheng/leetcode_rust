use std::i32;
//
pub trait Solution {
    fn train_ways(num: i32) -> i32;
}

pub struct TrainWays {}

impl Solution for TrainWays {
    fn train_ways(num: i32) -> i32 {
        let (mut a, mut b, mut sum) = (1, 1, 1);
        for _i in 0..num - 1 {
            sum = (a + b) % 1000000007;
            a = b;
            b = sum;
        }
        sum
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn judge_circle() {
        assert_eq!(TrainWays::train_ways(2), 2);
        assert_eq!(TrainWays::train_ways(5), 8);
    }
}
