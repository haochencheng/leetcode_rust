// 194. 二叉树的最近公共祖先

pub trait Solution {
    fn valid_palindrome(s: String) -> bool;

     fn valid_palindrome_sub(s: &[u8]) -> bool;
}


pub struct FindLengthOfLcis {}

impl Solution for FindLengthOfLcis {

    fn  valid_palindrome(s: String) -> bool  {
        
    }

    

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn judge_circle() {
        let s :String  = "aba".to_string();
        assert_eq!(FindLengthOfLcis::valid_palindrome(s),true);
        
        
    }
}
