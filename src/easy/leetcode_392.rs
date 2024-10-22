// 392. 判断子序列
// ace abcde true
// aec abcde false
// acau bcdccaaaadue false



use std::{collections::HashMap, option};

pub trait Solution {
    fn is_subsequence(s: String, t: String) -> bool ;
}


pub struct IsSubsequence {}

impl Solution for IsSubsequence {

    fn is_subsequence(s: String, t: String) -> bool {
        let s = s.chars().collect::<Vec<_>>();
        let t = t.chars().collect::<Vec<_>>();
        let (mut si, mut ti) = (0, 0);
        while si < s.len() && ti < t.len() {
            si += if s[si] == t[ti] { 1 } else { 0 };
            ti += 1;
        }
        si == s.len()
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn max_unique_split() {
        let s = String::from("abc");
        let t = String::from("ahbgdc");
        assert_eq!(IsSubsequence::is_subsequence(s, t),true);
        
        let s = String::from("aaaaaa");
        let t = String::from("bbaaaa");
        assert_eq!(IsSubsequence::is_subsequence(s, t),false);

        let s = String::from("ab");
        let t = String::from("baab");
        assert_eq!(IsSubsequence::is_subsequence(s, t),true);
    }
}
