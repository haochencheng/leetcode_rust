// LCR 182. 动态口令


pub trait Solution {
    fn dynamic_password(password: String, target: i32) -> String;

}


pub struct DynamicPassword {}

impl Solution for DynamicPassword {

    fn dynamic_password(password: String, target: i32) -> String{
        if password.is_empty() {
            return "".to_string();
        }
        let len =password.len();
        let mut result:Vec<char> = vec![];
        let chars = password.as_bytes();
        for i in target as usize..target as usize +len {
            let index = i as usize%len ;
            result.push(chars[index] as char);
        }
        // println!("result====={:#?}",result);
        return result.into_iter().collect();
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn judge_circle() {
        let password = "s3cur1tyC0d3".to_string();
        let target = 4;
        let result = "r1tyC0d3s3cu".to_string();
        assert_eq!(DynamicPassword::dynamic_password(password,target),result);
        
        
    }
}
