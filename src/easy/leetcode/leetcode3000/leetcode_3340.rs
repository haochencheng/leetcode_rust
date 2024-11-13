struct Solution {}

impl Solution {
    pub fn is_balanced(num: String) -> bool {
        let mut odd = 0;
        let mut even = 0;
        for (i, &byte) in num.as_bytes().iter().enumerate() {
            if i % 2 == 0 {
                odd += (byte - b'0') as i32;
            } else {
                even += (byte - b'0') as i32
            }
        }
        println!("odd:{},even:{}", odd, even);
        odd == even
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_balanced() {
        assert_eq!(Solution::is_balanced("1234".to_string()), false);
        assert_eq!(Solution::is_balanced("24123".to_string()), true);
    }
}
