struct Solution {}

impl Solution {
    pub fn max_absolute_sum(nums: Vec<i32>) -> i32 {
        let mut max_sum = 0; // 用于记录最大子数组和
        let mut min_sum = 0; // 用于记录最小子数组和
        let mut current_sum = 0; // 当前前缀和

        for &x in &nums {
            current_sum += x;
            // 更新最大和和最小和
            max_sum = max_sum.max(current_sum);
            min_sum = min_sum.min(current_sum);
        }

        // 返回绝对值的最大差
        max_sum - min_sum
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn max_absolute_sum() {
        let nums = vec![1, -3, 2, 3, -4];
        assert_eq!(Solution::max_absolute_sum(nums), 5);
        let nums = vec![2, -5, 1, -4, 3, -2];
        assert_eq!(Solution::max_absolute_sum(nums), 8);
    }
}
