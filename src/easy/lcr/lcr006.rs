use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map: HashMap<i32, usize> = HashMap::new();
        for i in 0..numbers.len() {
            if map.contains_key(&(target - numbers[i])) {
                return vec![
                    *map.get(&(target - numbers[i] as i32)).unwrap() as i32,
                    i as i32,
                ];
            }
            map.insert(numbers[i], i);
        }
        return Vec::new();
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn is_palindrome() {
        let nums = vec![1, 2, 4, 6, 10];
        assert_eq!(Solution::two_sum(nums, 8), vec![1, 3]);
    }
}
