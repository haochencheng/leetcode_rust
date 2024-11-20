use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn max_freq(s: String, max_letters: i32, min_size: i32, _max_size: i32) -> i32 {
        let min_size = min_size as usize;
        let max_letters = max_letters as usize;
        let n = s.len();

        let mut hash_map: HashMap<char, i32> = HashMap::new();
        let mut frequency_map = HashMap::new();
        let chars: Vec<char> = s.chars().collect();

        // Initialize the first window
        for i in 0..min_size {
            *hash_map.entry(chars[i]).or_insert(0) += 1;
        }
        if hash_map.len() <= max_letters {
            let substring: String = chars[0..min_size].iter().collect();
            *frequency_map.entry(substring).or_insert(0) += 1;
        }

        // Slide the window
        for i in min_size..n {
            let entering = chars[i];
            let exiting = chars[i - min_size];

            // Update the character frequency in the hash_map
            *hash_map.entry(entering).or_insert(0) += 1;
            if let Some(count) = hash_map.get_mut(&exiting) {
                *count -= 1;
                if *count == 0 {
                    hash_map.remove(&exiting);
                }
            }

            // Check the new window substring
            if hash_map.len() <= max_letters {
                let substring: String = chars[i - min_size + 1..i + 1].iter().collect();
                *frequency_map.entry(substring).or_insert(0) += 1;
            }
        }

        // Return the maximum frequency found
        *frequency_map.values().max().unwrap_or(&0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn max_unique_split() {
        let s = "aababcaab".to_string();
        let max_letters = 2;
        let min_size = 3;
        let max_size = 4;
        assert_eq!(Solution::max_freq(s, max_letters, min_size, max_size), 2);
    }
}
