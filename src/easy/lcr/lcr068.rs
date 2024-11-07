//
pub trait Solution {
    fn search_insert(nums: Vec<i32>, target: i32) -> i32;
}

pub struct SearchInsert {}

impl Solution for SearchInsert {
    fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        if target < nums[0] {
            return 0;
        }
        if target > nums[nums.len() - 1] {
            return nums.len() as i32;
        }

        for i in 0..nums.len() - 1 {
            if nums[i] < target && nums[i + 1] >= target {
                return (i + 1) as i32;
            }
        }
        return 0;
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn peak_index_in_mountain_array() {
        let arr = vec![1, 3, 5, 6];
        assert_eq!(SearchInsert::search_insert(arr, 5), 2);
        let arr = vec![1, 3, 5, 6];
        assert_eq!(SearchInsert::search_insert(arr, 2), 1);
        let arr = vec![1, 3, 5, 6];
        assert_eq!(SearchInsert::search_insert(arr, 7), 4);
    }
}
