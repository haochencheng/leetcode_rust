use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn maximum_subarray_sum(nums: Vec<i64>, k: i64) -> i64 {
        let k = k as usize;
        let n = nums.len();

        let mut hash_map: HashMap<i64, i64> = HashMap::new();
        let mut ans: i64 = 0;
        let mut current_sum: i64 = 0;
        // Initialize the first window
        for &num in &nums[0..k] {
            *hash_map.entry(num as i64).or_default() += 1;
            current_sum += num as i64;
        }

        // Calculate the sum of k * v for the first window
        if hash_map.len() == k {
            ans = ans.max(current_sum);
        }
        // Slide the window
        for i in k..n {
            let entering = nums[i] as i64;
            let exiting = nums[i - k] as i64;

            // Add the new element to the window
            *hash_map.entry(entering).or_default() += 1;
            current_sum += entering;

            // Subtract the element that is sliding out of the window
            if let Some(v) = hash_map.get_mut(&exiting) {
                *v -= 1;
                // If the value reaches 0, remove the entry
                if *v == 0 {
                    hash_map.remove(&exiting);
                }
            }
            current_sum -= exiting;
            // Incrementally update the sum
            if hash_map.len() == k {
                ans = ans.max(current_sum);
            }
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn max_unique_split() {
        let nums = vec![4, 4, 4];
        let k = 3;
        let ans = 0;
        assert_eq!(Solution::maximum_subarray_sum(nums, k), ans);

        let nums = vec![9, 9, 9, 1, 2, 3];
        let k = 3;
        let ans = 12;
        assert_eq!(Solution::maximum_subarray_sum(nums, k), ans);

        let nums = vec![1, 5, 4, 2, 9, 9, 9];
        let k = 3;
        let ans = 15;
        assert_eq!(Solution::maximum_subarray_sum(nums, k), ans);
    }
}
