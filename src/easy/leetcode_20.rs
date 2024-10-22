// 20. 有效的括号
pub trait Solution {

    fn is_valid(s: String) -> bool ;
        

}


pub struct IsValid {}

impl Solution for IsValid {

    fn is_valid(s: String) -> bool {
        if s.len()%2!=0 {
            return false;
        }
        let mut _match = vec![];
        for _b in s.bytes() {
            match _b {
                b'(' => _match.push(b')'),
                b'[' => _match.push(b']'),
                b'{' => _match.push(b'}'),
                _ => {
                    if _match.pop()!= Some(_b) {
                        return false;
                    }
                }
            }
        }
        return _match.is_empty();
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn max_unique_split() {
        let s = String::from("()");
        assert_eq!(<IsValid as Solution>::is_valid(s),true);
        let s = String::from("([])");
        assert_eq!(<IsValid as Solution>::is_valid(s),true);
        let s = String::from("()[]{}");
        assert_eq!(<IsValid as Solution>::is_valid(s),true);
        let s = String::from("([)]");
        assert_eq!(<IsValid as Solution>::is_valid(s),false);
    }
}
