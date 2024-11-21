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
    fn delete_and_earn() {
        let nums = vec![3, 4, 2];
        assert_eq!(Solution::delete_and_earn(nums), 6);
        let nums = vec![2, 2, 3, 3, 3, 4];
        assert_eq!(Solution::delete_and_earn(nums), 9);
    }
}
