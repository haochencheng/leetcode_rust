use core::num;
use std::collections::HashMap;

// 不简单的简单题
pub trait Solution {
    fn can_partition(nums: Vec<i32>) -> bool;
}

pub struct CanPartition {}

impl Solution for CanPartition {
    fn can_partition(mut nums: Vec<i32>) -> bool {
        if nums.is_empty() {
            return true;
        }
        let sum: i32 = nums.iter().sum();
        if sum % 2 != 0 {
            return false;
        }
        nums.sort_unstable();
        let (n, target_sum) = (nums.len(), sum as usize / 2);
        let mut dp = vec![0; target_sum + 1];
        for i in 1..n {
            for j in (nums[i - 1] as usize..=target_sum).rev() {
                dp[j] = i32::max(dp[j], nums[i - 1] + dp[j - nums[i - 1] as usize]);
                if dp[j] == target_sum as i32 {
                    return true;
                }
            }
        }
        return false;
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn path_encryption() {
        let nums = vec![1, 5, 11, 5];
        let result = true;
        assert_eq!(CanPartition::can_partition(nums), result);
    }
}
