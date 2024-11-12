use std::collections::HashMap;
use std::i32;

struct Solution {}

impl Solution {
    pub fn minimum_switching_times(source: Vec<Vec<i32>>, target: Vec<Vec<i32>>) -> i32 {
        if source.is_empty() {
            return 0;
        }
        let mut ans = 0;
        let mut source_map: HashMap<i32, i32> = HashMap::new();
        for i in 0..source.len() {
            for j in 0..source[i].len() {
                *source_map.entry(source[i][j]).or_default() += 1;
            }
        }
        for i in 0..target.len() {
            for j in 0..target[i].len() {
                *source_map.entry(target[i][j]).or_default() -= 1;
            }
        }
        i32::abs(
            source_map
                .iter()
                .map(|(_, &val)| val)
                .filter(|x| x < &0)
                .sum(),
        )
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn maximum_score() {
        let source = vec![vec![1, 2, 3], vec![3, 4, 5]];
        let target = vec![vec![1, 3, 5], vec![2, 3, 4]];
        assert_eq!(Solution::minimum_switching_times(source, target), 0);
    }
}
