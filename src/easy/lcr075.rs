use std::collections::HashMap;

//
pub trait Solution {
    fn relative_sort_array(arr1: Vec<i32>, arr2: Vec<i32>) -> Vec<i32>;
}

pub struct RelativeSortArray {}

impl Solution for RelativeSortArray {
    fn relative_sort_array(arr1: Vec<i32>, arr2: Vec<i32>) -> Vec<i32> {
        if arr1.is_empty() {
            return Vec::new();
        }

        let mut vec_last = Vec::new();
        let mut hash_map: HashMap<i32, i32> = HashMap::new();
        let mut hash_map_2: HashMap<i32, i32> = HashMap::new();

        for &num in &arr2 {
            hash_map_2.insert(num, 0);
        }

        // Count occurrences of each element in arr1, separating out those not in arr2
        for &num in &arr1 {
            if hash_map_2.contains_key(&num) {
                *hash_map.entry(num).or_default() += 1;
            } else {
                vec_last.push(num);
            }
        }

        let mut ans = Vec::new();

        // Add elements from arr2 based on counts in hash_map
        for ele in arr2 {
            if let Some(&count) = hash_map.get(&ele) {
                for _ in 0..count {
                    ans.push(ele);
                }
            }
        }

        // Sort remaining elements not in arr2 and append them to ans
        vec_last.sort_unstable();
        ans.append(&mut vec_last);

        ans
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn path_encryption() {
        let (arr1, arr2) = (
            vec![2, 3, 1, 3, 2, 4, 6, 7, 9, 2, 19],
            vec![2, 1, 4, 3, 9, 6],
        );
        let result = vec![2, 2, 2, 1, 4, 3, 3, 9, 6, 7, 19];
        assert_eq!(RelativeSortArray::relative_sort_array(arr1, arr2), result);
    }
}
