// 674. 最长连续递增序列

pub trait Solution {
    fn find_length_of_lcis(nums: Vec<i32>) -> i32;
}

pub struct FindLengthOfLcis {}

impl Solution for FindLengthOfLcis {
    fn find_length_of_lcis(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        let mut record = 1;
        let mut max = 1;
        for i in 1..nums.len() {
            if nums[i] > nums[i - 1] {
                record += 1;
                max = max.max(record);
            } else {
                record = 1;
            }
        }
        max as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn judge_circle() {
        let nums: Vec<i32> = vec![1, 3, 5, 4, 7];
        let result = 3;
        assert_eq!(FindLengthOfLcis::find_length_of_lcis(nums), result);
        let nums: Vec<i32> = vec![2, 2, 2, 2, 2];
        let result = 1;
        assert_eq!(FindLengthOfLcis::find_length_of_lcis(nums), result);
        let nums: Vec<i32> = vec![1, 3, 5, 7];
        let result = 4;
        assert_eq!(FindLengthOfLcis::find_length_of_lcis(nums), result);
    }
}
