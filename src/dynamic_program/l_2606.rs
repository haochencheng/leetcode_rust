use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn maximum_cost_substring(s: String, chars: String, vals: Vec<i32>) -> i32 {
        let mut char_map = HashMap::new();
        for (i, ele) in chars.as_bytes().iter().enumerate() {
            char_map.insert(ele, i);
        }
        let mut pre: i32 = 0;
        let mut ans: i32 = 0;
        for &ele in s.as_bytes() {
            let mut val: i32 = 0;
            if let Some(x) = char_map.get(&ele) {
                val = vals[*x] as i32;
            } else {
                val = (ele - b'a' + 1) as i32;
            }
            // println!("val===={}", val);
            pre = (val + pre).max(val);
            // println!("pre====={}", pre);
            ans = ans.max(pre);
        }
        ans
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn max_sub_array() {
        let s = "adaa".to_string();
        let chars = "d".to_string();
        let vals = vec![-1000];
        assert_eq!(Solution::maximum_cost_substring(s, chars, vals), 2);
    }
}
