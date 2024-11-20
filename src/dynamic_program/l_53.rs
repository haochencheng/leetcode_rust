use core::num;

struct Solution {}

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut ans = nums[0];
        let mut pre = 0;
        for x in nums {
            pre = x.max(x + pre);
            ans = ans.max(pre)
        }
        ans
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn max_sub_array() {
        let nums = vec![-2, 1, -3, 4, -1, 2, 1, -5, 4];
        assert_eq!(Solution::max_sub_array(nums), 6);
    }
}
