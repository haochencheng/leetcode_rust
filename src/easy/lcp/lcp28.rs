use std::i32;

struct Solution {}

impl Solution {
    pub fn purchase_plans(nums: Vec<i32>, target: i32) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        let mut sort_nums = nums;
        sort_nums.sort_unstable();
        let mut ans = 0;
        let (mut i, mut j) = (0, sort_nums.len() - 1);
        while i < j {
            if sort_nums[i] + sort_nums[j] > target {
                j -= 1;
            } else {
                ans += j - i;
                i += 1;
            }
        }
        (ans % 1000000007) as i32
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn maximum_score() {
        let nums = vec![2, 5, 3, 5];
        let target = 6;
        assert_eq!(Solution::purchase_plans(nums, target), 1);
    }
}
