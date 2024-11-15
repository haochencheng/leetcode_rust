use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn max_sum(nums: Vec<i32>, m: i32, k: i32) -> i64 {
        let k = k as usize;
        let m = m as usize;
        let n = nums.len();

        let mut hash_map: HashMap<i64, i64> = HashMap::new();
        let mut ans: i64 = 0;

        // Initialize the first window
        for &num in &nums[0..k] {
            *hash_map.entry(num as i64).or_default() += 1;
        }

        // Calculate the sum of k * v for the first window
        if hash_map.len() >= m {
            ans = ans.max(hash_map.iter().map(|(k, v)| k * v).sum());
        }

        let mut current_sum: i64 = 0;

        // Slide the window
        for i in k..n {
            let entering = nums[i] as i64;
            let exiting = nums[i - k] as i64;

            // Add the new element to the window
            *hash_map.entry(entering).or_default() += 1;

            // Subtract the element that is sliding out of the window
            if let Some(v) = hash_map.get_mut(&exiting) {
                *v -= 1;
                // If the value reaches 0, remove the entry
                if *v == 0 {
                    hash_map.remove(&exiting);
                }
            }

            // Incrementally update the sum
            current_sum = hash_map.iter().map(|(k, v)| k * v).sum();
            if hash_map.len() >= m {
                ans = ans.max(current_sum);
            }
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn max_unique_split() {
        let nums = vec![
            381392175, 381392175, 144736575, 242819048, 766897549, 670848150, 545845469, 644989153,
            466671626, 936840598, 152286072, 949893267, 325118520, 325036947, 32029822, 807365516,
            496118126, 388002456, 257894534, 176095279, 32029822, 496118126, 388002456, 732365411,
            750845657, 182863811, 263806735, 638092084, 154122216, 732365411, 263806735, 624782446,
            600405222, 129512510, 878050800, 796594142, 297629419, 437801126, 793363205, 883701190,
            476561334, 40761152, 791860142, 437801126, 793363205, 351470856, 17611748, 275263561,
            553075279, 495507544, 595370881, 351470856, 595370881, 559562713, 294547465, 275061217,
            972193241, 420257967, 694516619, 282055519, 485496050, 393889829, 932590687, 488622638,
            400267521, 400267521, 43569784, 531151597, 597592334, 997387925, 59635229, 542331691,
            389237829, 765469865, 834235567, 816188611, 177316080, 313260986, 765469865, 840849786,
            452657566, 911490390, 466395411, 818285290, 933121159, 452657566, 933121159, 933121159,
        ];
        let m = 1;
        let k = 6;
        assert_eq!(Solution::max_sum(nums, m, k), 4536701744);
        let nums = vec![1, 2, 2];
        let m = 2;
        let k = 2;
        assert_eq!(Solution::max_sum(nums, m, k), 3);
        let nums = vec![5, 9, 9, 2, 4, 5, 4];
        let m = 1;
        let k = 3;
        assert_eq!(Solution::max_sum(nums, m, k), 23);
    }
}
