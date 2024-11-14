struct Solution;

impl Solution {
    pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
        if nums.is_empty() {
            return 0.0;
        }
        let mut sum = 0;
        let mut ans: f64 = f64::MIN;
        for i in 0..nums.len() {
            sum += nums[i];
            if i < k as usize - 1 {
                continue;
            }
            let avg = sum as f64 / k as f64;
            // println!("v ==={},avg ===={}", sum, avg);
            ans = ans.max(avg);
            sum -= nums[i - (k - 1) as usize];
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn max_unique_split() {
        let nums = vec![1, 12, -5, -6, 50, 3];
        let k = 4;
        assert_eq!(Solution::find_max_average(nums, k), 12.75);
    }
}
