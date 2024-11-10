use std::i32;

struct Solution {}

impl Solution {
    pub fn get_minimum_time(time: Vec<i32>, fruits: Vec<Vec<i32>>, limit: i32) -> i32 {
        if time.is_empty() || fruits.is_empty() {
            return 0;
        }
        let mut ans = 0;
        for i in 0..fruits.len() {
            // println!("fruits[i====={:#?}", fruits[i][1]);
            let count = fruits[i][1] as f32 / limit as f32;
            // println!("count===={}", count.ceil());
            let t = time[fruits[i][0] as usize];

            ans += count.ceil() as i32 * t;
        }
        ans
    }
}

#[cfg(test)]
mod tests {

    use std::vec;

    use super::*;

    #[test]
    fn min_num_booths() {
        let time = vec![2, 3, 2];
        let fruits = vec![vec![0, 2], vec![1, 4], vec![2, 1]];
        let limit = 3;
        assert_eq!(Solution::get_minimum_time(time, fruits, limit), 10);
        let time = vec![1];
        let fruits = vec![vec![0, 3], vec![0, 5]];
        let limit = 2;
        assert_eq!(Solution::get_minimum_time(time, fruits, limit), 5);
    }
}
