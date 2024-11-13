use std::collections::{BinaryHeap, HashMap};

struct Solution {}

impl Solution {
    pub fn find_x_sum(nums: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
        if nums.is_empty() {
            return Vec::new();
        }
        let mut ans = Vec::new();
        let k = k as usize;
        for i in 0..nums.len() - k + 1 {
            let sub = &nums[i..i + k];
            Self::sub(sub, x, &mut ans);
        }

        ans
    }

    fn sub(nums: &[i32], x: i32, ans: &mut Vec<i32>) {
        // println!("nums====={:#?}", nums);
        let mut dequeue = BinaryHeap::new();
        let mut map: HashMap<i32, i32> = HashMap::new();
        for i in 0..nums.len() {
            *map.entry(nums[i]).or_default() += 1;
        }

        for (_key, v) in &map {
            dequeue.push(v);
        }
        let mut inverted_map: HashMap<i32, Vec<i32>> = HashMap::new();

        for (key, v) in &map {
            inverted_map.entry(*v).or_insert_with(Vec::new).push(*key);
        }
        let mut sum = 0;
        let mut mut_x = x;
        // print!("dequeue======{:#?}", dequeue);
        // println!("inverted_map:{:#?}", inverted_map);
        while !dequeue.is_empty() && mut_x > 0 {
            let count = dequeue.pop().unwrap();
            let mut arr = inverted_map.remove(count);
            if let Some(ref mut val) = &mut arr {
                println!("va;====={:#?}", val);
                val.sort_by(|a, b| b.cmp(a));
                for i in val {
                    sum += *i * count;
                    mut_x -= 1;
                    if mut_x <= 0 {
                        break;
                    }
                }
            }
        }
        ans.push(sum);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_balanced() {
        let nums = vec![1, 1, 2, 2, 3, 4, 2, 3];
        let k = 6;
        let x = 2;

        let ans = vec![6, 10, 12];
        assert_eq!(Solution::find_x_sum(nums, k, x), ans);
    }
}
