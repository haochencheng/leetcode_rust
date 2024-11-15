use std::collections::BTreeMap;

struct Solution {}

impl Solution {
    pub fn get_subarray_beauty(nums: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
        let k = k as usize;
        let x = x as usize;
        let mut result = Vec::new();
        let mut window = BTreeMap::new();

        // 初始化第一个窗口
        for &num in &nums[0..k] {
            *window.entry(num).or_insert(0) += 1;
        }
        let min = Self::get_xth_smallest(&window, x);
        // 获取第一个窗口的第 x 小值
        result.push(if min < 0 { min } else { 0 });

        // 滑动窗口
        for i in k..nums.len() {
            // 移除滑出窗口的元素
            let out_num = nums[i - k];
            if let Some(count) = window.get_mut(&out_num) {
                *count -= 1;
                if *count == 0 {
                    window.remove(&out_num);
                }
            }

            // 添加新的元素
            let in_num = nums[i];
            *window.entry(in_num).or_insert(0) += 1;

            // 获取当前窗口的第 x 小值
            let min = Self::get_xth_smallest(&window, x);
            result.push(if min < 0 { min } else { 0 });
        }

        result
    }

    // 获取有序窗口中的第 x 小元素
    fn get_xth_smallest(window: &BTreeMap<i32, i32>, x: usize) -> i32 {
        let mut count = 0;
        for (&key, &freq) in window {
            count += freq as usize;
            if count >= x {
                return key;
            }
        }
        0 // 如果找不到第 x 小元素，返回 0（根据题意调整返回值）
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn max_unique_split() {
        let nums = vec![1, -1, -3, -2, 3];
        let k = 3;
        let x = 2;
        let ans = vec![-1, -2, -2];
        assert_eq!(Solution::get_subarray_beauty(nums, k, x), ans);
    }
}
