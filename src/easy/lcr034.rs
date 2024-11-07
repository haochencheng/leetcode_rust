struct Solution {}

impl Solution {
    pub fn is_alien_sorted(words: Vec<String>, order: String) -> bool {
        use std::collections::HashMap;
        let ch2idx: HashMap<char, usize> = order.chars().zip((0..)).collect();
        // println!("{:?}, {}", ch2idx, ch2idx.len());
        // 翻译一遍
        let nums: Vec<Vec<usize>> = words
            .iter()
            .map(|word| word.chars().map(|ch| ch2idx[&ch]).collect::<Vec<usize>>())
            .collect();
        // println!("{:?}", nums);
        nums.windows(2)
            // .inspect(|slice| println!("{:?}", slice))
            .all(|slice| match slice {
                [prev, curr] => prev <= curr,
                _ => false,
            })
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn moving_average() {
        let words = vec!["hello".to_string(), "leetcode".to_string()];
        let order = "hlabcdefgijkmnopqrstuvwxyz".to_string();
        assert!(Solution::is_alien_sorted(words, order));
    }
}
