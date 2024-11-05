// //
pub trait Solution {
    fn path_encryption(path: String) -> String;
}

pub struct PathEncryption {}

impl Solution for PathEncryption {
    fn path_encryption(path: String) -> String {
        if path.is_empty() {
            return path;
        }
        let mut ans = String::from("");
        path.chars().for_each(|x| match x {
            '.' => ans += " ",
            _ => ans += &x.to_string(),
        });
        // println!("path ===={:#?}", ans);
        return ans;
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn path_encryption() {
        let path = "a.aef.qerf.bb".to_string();
        let result = "a aef qerf bb".to_string();
        assert_eq!(PathEncryption::path_encryption(path), result);
    }
}
