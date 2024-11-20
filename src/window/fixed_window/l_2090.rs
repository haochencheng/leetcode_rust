struct Solution;

impl Solution {
    pub fn get_averages(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let n = nums.len();
        let k = k as usize;
        let mut ans = vec![-1; n];

        // Return early if the window size is larger than the array length
        if k * 2 + 1 > n {
            return ans;
        }

        // Initial sum for the first window
        let mut sum: i32 = nums.iter().take(2 * k + 1).sum();

        // Calculate the average for each center position from index `k` to `n - k - 1`
        for i in k..=n - k - 1 {
            ans[i] = sum / (k as i32 * 2 + 1);

            // Slide the window to the right by removing the leftmost element and adding the next element
            if i < n - k - 1 {
                sum += nums[i + k + 1] - nums[i - k];
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
        let nums = vec![7, 4, 3, 9, 1, 8, 5, 2, 6];
        let k = 3;
        let ans = vec![-1, -1, -1, 5, 4, 4, -1, -1, -1];
        assert_eq!(Solution::get_averages(nums, k,), ans);
    }
}
