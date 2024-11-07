use std::i32;

//
pub trait Solution {
    fn find_maximum_xor(nums: Vec<i32>) -> i32;
}

pub struct FindMaximumXor {}

impl Solution for FindMaximumXor {
    fn find_maximum_xor(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        if nums.len() == 1 {
            return 0;
        }
        let mut max = i32::MIN;
        for i in 0..nums.len() - 1 {
            for j in i + 1..nums.len() {
                max = max.max(nums[i] ^ nums[j]);
            }
        }
        max
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn peak_index_in_mountain_array() {
        let nums = vec![3, 10, 5, 25, 2, 8];
        assert_eq!(FindMaximumXor::find_maximum_xor(nums), 28);
        let nums = vec![0];
        assert_eq!(FindMaximumXor::find_maximum_xor(nums), 0);
        println!("====={}", 8 ^ 10);
        println!("====={}", 10 ^ 2);
        let nums = vec![8, 10, 2];
        assert_eq!(FindMaximumXor::find_maximum_xor(nums), 10);
    }
}
