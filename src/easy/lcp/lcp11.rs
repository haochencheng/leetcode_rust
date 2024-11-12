use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn num_ways(n: i32, relation: Vec<Vec<i32>>, k: i32) -> i32 {
        let mut map: HashMap<i32, Vec<i32>> = HashMap::new();
        // 将关系存入HashMap
        for rel in relation {
            map.entry(rel[0]).or_default().push(rel[1]);
        }
        // 从起点出发，通过DFS搜索路径
        Self::dfs(0, &map, k, n)
    }

    fn dfs(node: i32, map: &HashMap<i32, Vec<i32>>, steps_remaining: i32, target: i32) -> i32 {
        // 如果步数用完了，检查是否到达目标节点
        if steps_remaining == 0 {
            return if node == target - 1 { 1 } else { 0 };
        }

        // 如果当前节点没有子节点可走，返回0
        if let Some(neighbors) = map.get(&node) {
            // 累计所有子节点的路径数
            neighbors
                .iter()
                .map(|&next| Self::dfs(next, map, steps_remaining - 1, target))
                .sum()
        } else {
            0
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn maximum_score() {
        let n = 5;
        let relation = vec![
            vec![0, 2],
            vec![2, 1],
            vec![3, 4],
            vec![2, 3],
            vec![1, 4],
            vec![2, 0],
            vec![0, 4],
        ];
        let k = 3;
        assert_eq!(Solution::num_ways(n, relation, k), 3);
    }
}
