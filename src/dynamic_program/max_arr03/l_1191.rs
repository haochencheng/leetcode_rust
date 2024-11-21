struct Solution {}

impl Solution {
    pub fn k_concatenation_max_sum(arr: Vec<i32>, k: i32) -> i32 {
        const MOD: i32 = 1_000_000_007;

        // 辅助函数：计算最大子数组和（Kadane's Algorithm）
        fn max_subarray_sum(nums: &[i32]) -> i32 {
            let mut max_sum = 0;
            let mut current_sum = 0;
            for &x in nums {
                current_sum = (current_sum + x).max(0);
                max_sum = max_sum.max(current_sum);
            }
            max_sum
        }

        // 计算单个数组的最大子数组和
        let max_sum_single = max_subarray_sum(&arr);

        // 计算数组元素总和
        let total_sum: i64 = arr.iter().map(|&x| x as i64).sum();

        if k == 1 {
            // 如果只重复一次，直接返回单个数组的最大子数组和
            return max_sum_single % MOD;
        }

        // 计算双数组拼接的最大子数组和
        let mut extended_arr = vec![];
        extended_arr.extend_from_slice(&arr);
        extended_arr.extend_from_slice(&arr);
        let max_sum_double = max_subarray_sum(&extended_arr);

        if total_sum > 0 {
            // 如果数组和为正，可以跨越多次重复，取前缀和与后缀和
            let result = (max_sum_double as i64 + (k as i64 - 2) * total_sum) % MOD as i64;
            result as i32
        } else {
            // 如果数组和为负，则不需要跨越多次，直接返回双数组的最大子数组和
            max_sum_double % MOD
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn max_sub_array() {
        let arr = vec![1, 2];
        let k = 3;
        assert_eq!(Solution::k_concatenation_max_sum(arr, k), 9);
        let arr = vec![-1, -2];
        let k = 7;
        assert_eq!(Solution::k_concatenation_max_sum(arr, k), 0);
        let arr = vec![
            10000, 10000, 10000, 10000, 10000, 10000, 10000, 10000, 10000, 10000,
        ];
        let k = 100000;
        assert_eq!(Solution::k_concatenation_max_sum(arr, k), 999999937);
    }
}
