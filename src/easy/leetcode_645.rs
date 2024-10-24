// 645. 错误的集合

pub trait Solution {
    fn find_error_nums(nums: Vec<i32>) -> Vec<i32>;
}


pub struct FindErrorNums {}

impl Solution for FindErrorNums {
    fn find_error_nums(nums: Vec<i32>) -> Vec<i32> {
        if nums.is_empty(){
            return Vec::new();
        }
        let mut pre=nums[0];
        for i in 1..nums.len() {
            if nums[i] != pre+1 {
                return vec![pre,pre+1];
            }
            pre = nums[i];
        }
        return Vec::new();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_error_nums() {
        let nums = vec![1,2,2,4];
        let result = vec![2,3];
        assert_eq!(FindErrorNums::find_error_nums(nums),result);
        
    }
}
