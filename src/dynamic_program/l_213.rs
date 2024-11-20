use core::num;

struct Solution {}

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        // 198. 打家劫舍
        fn rob1(nums: &[i32], start: usize, end: usize) -> i32 {
            let mut f0 = 0;
            let mut f1 = 0;
            for i in start..end {
                let new_f = f1.max(f0 + nums[i]);
                f0 = f1;
                f1 = new_f;
            }
            f1
        }

        let n = nums.len();
        rob1(&nums, 1, n).max(nums[0] + rob1(&nums, 2, n - 1))
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn maximum_score() {
        let nums = vec![2, 3, 2];
        assert_eq!(Solution::rob(nums), 3);
        let nums = vec![1, 2, 3];
        assert_eq!(Solution::rob(nums), 3);
        let nums = vec![1, 2, 1, 1];
        assert_eq!(Solution::rob(nums), 3);
    }
}
