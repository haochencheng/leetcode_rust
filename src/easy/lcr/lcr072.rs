//
pub trait Solution {
    fn my_sqrt(x: i32) -> i32;
}

pub struct MySqrt {}

impl Solution for MySqrt {
    fn my_sqrt(x: i32) -> i32 {
        let (mut l, mut r) = (0, x);
        while l < r {
            let mid = 1 + (r - l) / 2 + l;
            if mid <= x / mid {
                l = mid;
            } else {
                r = mid - 1;
            }
        }
        return l;
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn path_encryption() {
        assert_eq!(MySqrt::my_sqrt(4), 2);
    }
}
