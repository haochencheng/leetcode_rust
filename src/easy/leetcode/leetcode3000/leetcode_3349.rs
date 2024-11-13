struct Solution {}

impl Solution {
    pub fn has_increasing_subarrays(nums: Vec<i32>, k: i32) -> bool {
        let mut pre = 0;
        let mut cur = 1;
        if k == 1 {
            return true;
        }
        for i in 1..nums.len() {
            if nums[i - 1] < nums[i] {
                cur += 1;
            } else {
                if pre >= k && cur >= k {
                    return true;
                }
                pre = cur;
                if pre >= 2 * k {
                    return true;
                }
                cur = 1;
                println!("pre===={}", pre);
            }
        }
        println!("pre===={},cur===={}", pre, cur);
        if pre >= k && cur >= k {
            return true;
        }
        return false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn max_unique_split() {
        // let nums = vec![2, 5, 7, 8, 9, 2, 3, 4, 3, 1];
        // let k = 3;
        // assert_eq!(Solution::has_increasing_subarrays(nums, k), true);

        // let nums = vec![-15, 19];
        // let k = 1;
        // assert_eq!(Solution::has_increasing_subarrays(nums, k), true);

        let nums = vec![-15, -13, 4, 7];
        let k = 2;
        assert_eq!(Solution::has_increasing_subarrays(nums, k), true);
    }
}
