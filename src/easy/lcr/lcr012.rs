struct Solution {}

impl Solution {
    pub fn pivot_index(nums: Vec<i32>) -> i32 {
        let sum: i32 = nums.iter().sum();
        let mut left = 0;
        for i in 0..nums.len() {
            if sum == nums[i] + 2 * left {
                return i as i32;
            }
            left += nums[i];
        }
        return -1;
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn is_palindrome() {
        let nums = vec![1, 7, 3, 6, 5, 6];
        assert_eq!(Solution::pivot_index(nums), 3);
    }
}
