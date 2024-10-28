// LCR 181. 字符串中的单词反转

pub trait Solution {
    fn reverse_message(message: String) -> String;

}


pub struct ReverseMessage {}

impl Solution for ReverseMessage {

    fn reverse_message(message: String) -> String{
        if message.is_empty() {
            return "".to_string();
        }
        let mut arr: Vec<&str>  = message.split(' ').collect();
        let mut ret = String::new();
        arr.reverse();
        for ele in &arr {
            if !ele.is_empty(){
                ret.push_str(*ele);
                ret.push(' ');
            }
        }
        ret.trim().to_string()
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn judge_circle() {
        let password = "the sky is blue".to_string();
        let result = "blue is sky the".to_string();
        assert_eq!(ReverseMessage::reverse_message(password),result);
        
        
    }
}
