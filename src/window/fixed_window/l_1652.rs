use std::{collections::HashMap, sync::Arc};

struct Solution;
impl Solution {
    pub fn decrypt(code: Vec<i32>, k: i32) -> Vec<i32> {
        let len: i32 = code.len() as i32;
        let mut res: Vec<i32> = vec![];
        for (idx, value) in code.iter().enumerate() {
            let mut sum: i32 = 0;
            if k > 0 {
                for i in 1..=k {
                    let loc: usize = ((idx as i32 + i as i32) % len as i32) as usize;
                    sum += code[loc];
                }
            } else if k < 0 {
                for i in 1..=k.abs() {
                    let mut loc: i32 = idx as i32 - i;
                    if loc < 0 {
                        loc += len as i32;
                    }
                    sum += code[loc as usize];
                }
            }
            res.push(sum);
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn max_unique_split() {
        let code = vec![5, 7, 1, 4];
        let k = 3;
        let ans = vec![12, 10, 16, 13];

        assert_eq!(Solution::decrypt(code, k), ans);
    }
}
