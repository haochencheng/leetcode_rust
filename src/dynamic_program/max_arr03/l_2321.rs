use std::i32;

struct Solution {}

impl Solution {
    pub fn maximums_spliced_array(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        fn maximums_spliced_array_sub(nums1: &Vec<i32>, nums2: &Vec<i32>) -> i32 {
            let mut pre = 0;
            let mut ans = 0;
            for i in 0..nums1.len() {
                let diff = nums2[i] - nums1[i];
                pre = diff.max(pre + diff);
                ans = ans.max(pre);
            }
            nums1.iter().sum::<i32>() + ans
        }
        maximums_spliced_array_sub(&nums1, &nums2).max(maximums_spliced_array_sub(&nums2, &nums1))
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn max_sub_array() {
        let nums1 = vec![60, 60, 60];
        let nums2 = vec![10, 90, 10];
        let k = 210;
        assert_eq!(Solution::maximums_spliced_array(nums1, nums2), k);
        let nums1 = vec![20, 40, 20, 70, 30];
        let nums2 = vec![50, 20, 50, 40, 20];
        let k = 220;
        assert_eq!(Solution::maximums_spliced_array(nums1, nums2), k);
    }
}
