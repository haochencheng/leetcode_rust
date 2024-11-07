use std::i32;

struct Solution {}

impl Solution {
    pub fn rune_reserve(runes: Vec<i32>) -> i32 {
        let mut arr = runes;
        arr.sort_unstable();
        println!("arr==={:#?}", arr);
        let mut max = 1;
        let mut next = 1;
        for i in 1..arr.len() {
            if arr[i - 1] + 1 == arr[i] || arr[i - 1] == arr[i] {
                next += 1;
                println!("next==={}", next)
            } else {
                max = max.max(next);
                next = 1;
            }
        }
        max = max.max(next);
        max
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn is_palindrome() {
        // let ans = vec![1, 3, 5, 4, 1, 7];
        // assert_eq!(Solution::rune_reserve(ans), 3);
        let ans = vec![1, 1, 3, 3, 2, 4];
        assert_eq!(Solution::rune_reserve(ans), 6);
    }
}
