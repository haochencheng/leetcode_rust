// 643. 子数组最大平均数 I
use core::f64;

pub trait Solution {
    fn find_max_average(nums: Vec<i32>, k: i32) -> f64;
}

pub struct FindMaxAverage {}

impl Solution for FindMaxAverage {
    fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
        if nums.is_empty() {
            return 0.0;
        }
        if nums.len() <= k as usize {
            let sum: i32 = nums.iter().sum();
            println!("sum====={}", sum);
            return sum as f64 / k as f64;
        }
        let mut result = f64::MIN;
        for index in 0..nums.len() - (k - 1) as usize {
            let mut sum = 0;
            for i in 0..k {
                sum += nums[index + i as usize];
            }
            // println!("sum========{}",sum);
            let avg = sum as f64 / k as f64;
            if avg > result {
                result = avg;
            }
        }
        return result.into();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn max_unique_split() {
        let nums = vec![1, 12, -5, -6, 50, 3];
        let k = 4;
        let result = 12.75;
        assert_eq!(FindMaxAverage::find_max_average(nums, k), result);
        let nums = vec![5];
        let k = 1;
        let result = 5.0;
        assert_eq!(FindMaxAverage::find_max_average(nums, k), result);
        let nums = vec![0, 1, 1, 3, 3];
        let k = 4;
        let result = 2.0;
        assert_eq!(FindMaxAverage::find_max_average(nums, k), result);
    }
}
