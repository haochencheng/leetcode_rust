use std::collections::HashMap;

pub trait Solution {
    fn dismantling_action(arr: String) -> char;
}

pub struct DismantlingAction {}

// LCR 169. 招式拆解 II
impl Solution for DismantlingAction {
    fn dismantling_action(arr: String) -> char {
        let mut map: HashMap<char, i32> = HashMap::new();

        // Count occurrences of each character
        for c in arr.chars() {
            *map.entry(c).or_insert(0) += 1;
        }

        // Find the first character with a count of 1
        for c in arr.chars() {
            if map[&c] == 1 {
                return c;
            }
        }

        ' '
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn judge_circle() {
        let arr = "abbccdeff";
        assert_eq!(
            DismantlingAction::dismantling_action(String::from(arr)).to_string(),
            "a"
        );
    }
}
