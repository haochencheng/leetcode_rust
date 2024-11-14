use core::num;

struct Solution;

impl Solution {
    pub fn minimum_recolors(blocks: String, k: i32) -> i32 {
        let blocks = blocks.as_bytes();
        let k = k as usize;
        let n = blocks.len();
        // Initial sum for the first window
        let mut sum = blocks[0..k].iter().filter(|x| *x == &b'W').count();
        let mut ans = sum;
        // println!("sum====={}", sum);
        for i in k..n {
            // Slide the window to the right by removing the leftmost element and adding the next element
            if blocks[i] == b'W' {
                sum += 1;
            }
            if blocks[i - k] == b'W' {
                sum -= 1;
            }
            ans = ans.min(sum);
        }
        ans as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn max_unique_split() {
        let blocks = "WBBWWBBWBW".to_string();
        let k = 7;
        let ans = 3;
        assert_eq!(Solution::minimum_recolors(blocks, k), ans);
        let blocks = "WBWBBBW".to_string();
        let k = 2;
        let ans = 0;
        assert_eq!(Solution::minimum_recolors(blocks, k), ans);

        let blocks = "WWBBBWBBBBBWWBWWWB".to_string();
        let k = 16;
        let ans = 6;
        assert_eq!(Solution::minimum_recolors(blocks, k), ans);
    }
}
