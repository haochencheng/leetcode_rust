struct Solution {}

impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let a = a.as_bytes();
        let b = b.as_bytes();
        let mut result = Vec::new();
        let mut carry = 0;

        let mut i = a.len() as i32 - 1;
        let mut j = b.len() as i32 - 1;

        while i >= 0 || j >= 0 || carry > 0 {
            let mut sum = carry;

            if i >= 0 {
                sum += (a[i as usize] - b'0') as i32;
                i -= 1;
            }
            if j >= 0 {
                sum += (b[j as usize] - b'0') as i32;
                j -= 1;
            }

            carry = sum / 2;
            result.push((sum % 2).to_string());
        }

        result.reverse();
        result.concat()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn is_palindrome() {
        assert_eq!(
            Solution::add_binary("11".to_string(), "10".to_string()),
            "101".to_string()
        );
    }
}
