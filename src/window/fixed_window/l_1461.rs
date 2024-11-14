use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn has_all_codes(s: String, k: i32) -> bool {
        let k = k as usize;
        if k > s.len() {
            return false;
        }
        // 计算k的所有可能
        fn get_all_k(arr: &[char], arr_ans: &[String]) -> Vec<String> {
            let mut ans = vec![];
            for x in arr_ans {
                for &y in arr {
                    ans.push(format!("{}{}", x, y));
                }
            }
            ans
        }
        let arr = ['0', '1'];
        let mut arr_ans = vec!["0".to_string(), "1".to_string()];
        for _ in 1..k {
            arr_ans = get_all_k(&arr, &arr_ans);
        }
        // arr_ans now contains all combinations of `num_digits` digits
        // for combination in &arr_ans {
        //     println!("combination======{}", combination);
        // }

        let mut map: HashMap<&String, i32> = HashMap::new();
        // sliding window
        let n = s.len();
        let s = &s[0..n];

        // let mut to_remove: Option<usize> = None;

        for i in k..=n {
            let window = &s[i - k..i];

            // Remove the marked element after the loop iteration if applicable
            // if let Some(index) = to_remove {
            //     arr_ans.remove(index);
            //     to_remove = None;
            // }
            for i in 0..arr_ans.len() {
                if &arr_ans[i] == window {
                    // to_remove = Some(i); // Mark for removal
                    *map.entry(&arr_ans[i]).or_default() += 1;
                    break;
                }
            }
        }
        println!("map===={:#?}", map);
        return map.len() == arr_ans.len();

        // let k = k as usize;
        // if s.len() < k {
        //     return false;
        // }
        // let mut tab = vec![false; 1 << k];
        // let mut s = s.into_bytes();
        // s.iter_mut().for_each(|b| *b -= b'0');
        // let mut cur = (0..k).fold(0, |c, i| c + ((s[i] as usize) << (k - i - 1)));
        // tab[cur] = true;
        // for i in k..s.len() {
        //     cur -= (s[i - k] as usize) << (k - 1);
        //     cur = (cur << 1) + s[i] as usize;
        //     tab[cur] = true;
        // }
        // tab.into_iter().all(|t| t)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn max_unique_split() {
        let s = "00110110".to_string();
        let k = 2;
        assert_eq!(Solution::has_all_codes(s, k), true);
        let s = "0000000".to_string();
        let k = 2;
        assert_eq!(Solution::has_all_codes(s, k), false);
    }
}
