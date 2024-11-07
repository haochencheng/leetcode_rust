//

use std::i32;

pub trait Solution {
    fn results_array(nums: Vec<i32>, k: i32) -> Vec<i32>;
}

pub struct ResultsArray {}

impl Solution for ResultsArray {
    fn results_array(nums: Vec<i32>, k: i32) -> Vec<i32> {
        if nums.is_empty() {
            return Vec::new();
        }
        if nums.len() == 1 || k == 1 {
            return nums;
        }
        let k = k as usize;
        let n = nums.len();
        let mut ans = vec![-1; n - k + 1];
        for i in 0..(n - k + 1) {
            let sub = &nums[i..i + k];
            let mut min = sub[0] - 1;
            let mut next = true;
            for ele in sub {
                if *ele != min + 1 {
                    next = false;
                    break;
                }
                min = *ele;
            }
            if next {
                ans[i] = sub[sub.len() - 1];
            }
        }
        return ans;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn max_unique_split() {
        // let nums = vec![1, 2, 3, 4, 3, 2, 5];
        // let k = 3;
        // let result = vec![3, 4, -1, -1, -1];
        // assert_eq!(ResultsArray::results_array(nums, k), result);
        // let nums = vec![2, 3];
        // let k = 2;
        // let result = vec![3];
        // assert_eq!(ResultsArray::results_array(nums, k), result);
        let nums = vec![1, 3, 4];
        let k = 2;
        let result = vec![-1, 4];
        assert_eq!(ResultsArray::results_array(nums, k), result);
    }
}
