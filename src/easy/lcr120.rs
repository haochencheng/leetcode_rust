use std::collections::HashMap;

// //
pub trait Solution {
    fn find_repeat_document(documents: Vec<i32>) -> i32;
}

pub struct FindRepeatDocument {}

impl Solution for FindRepeatDocument {
    fn find_repeat_document(documents: Vec<i32>) -> i32 {
        if documents.is_empty() {
            return 0;
        }
        let mut hash_map: HashMap<i32, i32> = HashMap::new();
        for ele in documents {
            *hash_map.entry(ele).or_default() += 1;
        }
        for (k, v) in hash_map {
            if v > 1 {
                return k;
            }
        }
        return 0;
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn path_encryption() {
        let documents = vec![2, 5, 3, 0, 5, 0];
        let result = 5;
        assert_eq!(FindRepeatDocument::find_repeat_document(documents), result);
    }
}
