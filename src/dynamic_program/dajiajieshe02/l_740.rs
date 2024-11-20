struct Solution {}

impl Solution {
    pub fn delete_and_earn(nums: Vec<i32>) -> i32 {
        let mut sum = vec![0; *nums.iter().max().unwrap() as usize + 1];
        nums.iter().for_each(|&n| sum[n as usize] += n);
        let (mut first, mut second) = (sum[0], sum[0].max(sum[1]));
        sum.iter().skip(2).for_each(|&val| {
            let cur = second.max(first + val);
            first = second;
            second = cur;
        });
        first.max(second)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn maximum_score() {
        let nums = vec![1, 2, 3, 1];
        assert_eq!(Solution::rob(nums), 4);
        let nums = vec![2, 7, 9, 3, 1];
        assert_eq!(Solution::rob(nums), 12);
    }
}
