use std::collections::HashMap;
// 1593. Split a String Into the Max Number of Unique Substrings

pub trait Solution {
    fn max_unique_split(s: String) -> i32;
}

pub struct MaxUniqueSplit {}

impl Solution for MaxUniqueSplit {
    fn max_unique_split(s: String) -> i32 {
        let mut hash_map: HashMap<String, i32> = HashMap::new();
        for ele in s.chars() {
            let ele_str = ele.to_string();
            let x = hash_map.get(&ele_str);
            match x {
                Some(x) => {
                    hash_map.insert(ele_str, x + 1);
                }
                None => {
                    hash_map.insert(ele_str, 1);
                }
            }
        }
        let max = hash_map.values().max().unwrap();
        println!("{}", max.to_string());
        let x = *max;
        return x;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn max_unique_split() {
        let input = "ababccc";
        assert_eq!(
            <MaxUniqueSplit as Solution>::max_unique_split(input.to_string()),
            3
        );
    }
}
