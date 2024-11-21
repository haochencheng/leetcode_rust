use std::i32;

struct Solution {}

impl Solution {
    pub fn max_subarray_sum_circular(nums: Vec<i32>) -> i32 {
        let mut min = 0;
        let mut min_ans = i32::MAX;
        let mut max = 0;
        let mut max_ans = i32::MIN;
        let mut sum = 0;
        for i in 0..nums.len() {
            let x = nums[i];
            max = (x + max).max(x);
            max_ans = max_ans.max(max);
            min = (x + min).min(x);
            min_ans = min_ans.min(min);
            sum += x;
        }
        if min_ans == sum {
            max_ans
        } else {
            max_ans.max(sum - min_ans)
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn max_sub_array() {
        let arr = vec![1, -2, 3, -2];
        let k = 3;
        assert_eq!(Solution::max_subarray_sum_circular(arr), k);
        let arr = vec![5, -3, 5];
        let k = 10;
        assert_eq!(Solution::max_subarray_sum_circular(arr), k);
        let arr = vec![-3, -2, -3];
        let k = -2;
        assert_eq!(Solution::max_subarray_sum_circular(arr), k);
    }
}
