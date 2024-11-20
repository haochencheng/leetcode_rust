struct Solution {}

impl Solution {
    pub fn maximum_total_damage(power: Vec<i32>) -> i64 {
        use std::collections::HashMap;

        // 统计每种伤害值的出现次数
        let mut cnt = HashMap::new();
        for &p in &power {
            *cnt.entry(p).or_insert(0) += 1;
        }

        // 获取所有唯一的伤害值并排序
        let mut unique_damage: Vec<i32> = cnt.keys().cloned().collect();
        unique_damage.sort();

        // 定义递归函数，使用缓存优化
        fn dfs(
            i: usize,
            unique_damage: &[i32],
            cnt: &HashMap<i32, i32>,
            memo: &mut HashMap<usize, i64>,
        ) -> i64 {
            if i == usize::MAX {
                // Base case: 如果索引超出范围
                return 0;
            }

            // 检查缓存
            if let Some(&res) = memo.get(&i) {
                return res;
            }

            let x = unique_damage[i];
            let mut j = i;
            // 找到最远的 j，使得 unique_damage[j] < x - 2
            while j > 0 && unique_damage[j - 1] >= x - 2 {
                j -= 1;
            }

            // 计算最大值
            let result = dfs(i.wrapping_sub(1), unique_damage, cnt, memo).max(
                dfs(j.wrapping_sub(1), unique_damage, cnt, memo)
                    + x as i64 * (*cnt.get(&x).unwrap() as i64),
            );

            // 缓存结果
            memo.insert(i, result);

            result
        }

        // 调用递归函数，从最后一个元素开始
        let mut memo = HashMap::new();
        dfs(
            unique_damage.len().wrapping_sub(1),
            &unique_damage,
            &cnt,
            &mut memo,
        )
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn maximum_total_damage() {
        let power = vec![1, 1, 3, 4];
        assert_eq!(Solution::maximum_total_damage(power), 6);

        let power = vec![5, 1, 4];
        assert_eq!(Solution::maximum_total_damage(power), 6);

        let power = vec![7, 1, 6, 3];
        assert_eq!(Solution::maximum_total_damage(power), 10);

        let power = vec![1, 1, 1, 1, 1, 1];
        assert_eq!(Solution::maximum_total_damage(power), 6);
    }
}
