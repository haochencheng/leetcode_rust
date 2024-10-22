
use std::collections::HashMap;
// 1593. Split a String Into the Max Number of Unique Substrings

pub trait Solution{
    fn max_unique_split(s: String) -> i32 ;

}

impl dyn Solution {
    pub fn max_unique_split(s: String) -> i32 {
        // let hash_map: HashMap<key: String, count: i32> = HashMap::new();
        // hashMap.entry(key).or_default() += 1;
        // let x = hashMap.values().cloned().max().unwrap();
        // println!(x)
        1
    }

}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn max_unique_split() {
        let input="ababccc";
        assert_eq!(Solution::max_unique_split(input.to_string()), 5);
    }

}
