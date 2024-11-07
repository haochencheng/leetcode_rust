use std::i32;

//
pub trait Solution {
    fn peak_index_in_mountain_array(arr: Vec<i32>) -> i32;
}

pub struct PeakIndexInMountainArray {}

impl Solution for PeakIndexInMountainArray {
    fn peak_index_in_mountain_array(arr: Vec<i32>) -> i32 {
        if arr.is_empty() {
            return 0;
        }
        for i in 1..arr.len() {
            if arr[i - 1] < arr[i] && arr[i] > arr[i + 1] {
                return i as i32;
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
        let arr = vec![0, 1, 0];
        assert_eq!(
            PeakIndexInMountainArray::peak_index_in_mountain_array(arr),
            1
        );
        let arr = vec![1, 3, 5, 4, 2];
        assert_eq!(
            PeakIndexInMountainArray::peak_index_in_mountain_array(arr),
            2
        );
    }
}
