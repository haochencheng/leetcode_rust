struct Solution;

impl Solution {
    pub fn num_of_subarrays(arr: Vec<i32>, k: i32, threshold: i32) -> i32 {
        if arr.is_empty() {
            return 0;
        }
        let mut sum = 0;
        let mut ans = 0;
        for i in 0..arr.len() {
            sum += arr[i];
            if i < k as usize - 1 {
                continue;
            }
            let avg = sum / k;
            // println!("v ==={},avg ===={}", sum, avg);
            if threshold <= avg {
                ans += 1;
            }
            sum -= arr[i - (k - 1) as usize];
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn max_unique_split() {
        let arr = vec![2, 2, 2, 2, 5, 5, 5, 8];
        let threshold = 4;
        let k = 3;
        assert_eq!(Solution::num_of_subarrays(arr, k, threshold), 3);
    }
}
