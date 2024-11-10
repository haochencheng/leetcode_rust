use std::{collections::HashMap, i32};

struct Solution {}

impl Solution {
    pub fn min_num_booths(demand: Vec<String>) -> i32 {
        if demand.is_empty() {
            return 0;
        }
        let mut ans = HashMap::new();
        for ele in demand {
            let mut map: HashMap<u8, i32> = HashMap::new();
            for ele in ele.bytes() {
                *map.entry(ele).or_default() += 1;
                if !ans.contains_key(&ele) {
                    ans.insert(ele, 1);
                }
            }
            for key in map.keys() {
                let max = map.get(key).unwrap();
                if map.get(key).unwrap() > ans.get(key).unwrap() {
                    ans.insert(*key, *max);
                }
            }
        }
        ans.values().sum()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn min_num_booths() {
        let demand = vec!["acd".to_string(), "bed".to_string(), "accd".to_string()];
        assert_eq!(Solution::min_num_booths(demand), 6);
    }
}
