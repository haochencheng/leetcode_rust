use std::i32;

struct Solution {}

impl Solution {
    pub fn supply_wagon(mut supplies: Vec<i32>) -> Vec<i32> {
        if supplies.is_empty() {
            return Vec::new();
        }

        let n = supplies.len();

        while supplies.len() > n / 2 {
            let mut min_sum = i32::MAX;
            let mut min_index = 0;

            for i in 0..supplies.len() - 1 {
                let sum = supplies[i] + supplies[i + 1];

                if sum < min_sum {
                    min_sum = sum;
                    min_index = i;
                }
            }

            supplies[min_index] += supplies[min_index + 1];
            supplies.remove(min_index + 1);
        }

        supplies
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn is_palindrome() {
        let supplies = vec![7, 3, 6, 1, 8];
        let ans = vec![10, 15];
        assert_eq!(Solution::supply_wagon(supplies), ans);
        let supplies = vec![1, 3, 1, 5];
        let ans = vec![5, 5];
        assert_eq!(Solution::supply_wagon(supplies), ans);
        let supplies = vec![6, 2, 2, 6, 9, 8, 5, 7];
        //  2,2,5,6, 6, 7, 8,9
        let ans = vec![10, 15, 8, 12];
        assert_eq!(Solution::supply_wagon(supplies), ans);
    }
}
