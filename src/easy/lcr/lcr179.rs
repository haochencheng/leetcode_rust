// LCR 179. 查找总价格为目标值的两个商品

pub trait Solution {
    fn two_sum(price: Vec<i32>, target: i32) -> Vec<i32>;
}

pub struct TwoSum {}

impl Solution for TwoSum {
    fn two_sum(price: Vec<i32>, target: i32) -> Vec<i32> {
        if price.is_empty() {
            return Vec::new();
        }
        let mut l = 0;
        let mut r = price.len() - 1;
        while l < price.len() - 1 && l < r {
            let sum = price[l] as usize + price[r] as usize;
            if sum > target as usize {
                r -= 1;
            } else if sum < target as usize {
                l += 1;
            } else {
                return vec![price[l], price[r]];
            }
        }
        return Vec::new();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn judge_circle() {
        let price: Vec<i32> = vec![3, 9, 12, 15];
        let target = 18;
        let result = vec![3, 15];
        assert_eq!(TwoSum::two_sum(price, target), result);
        let price: Vec<i32> = vec![8, 21, 27, 34, 52, 66];
        let target = 61;
        let result = vec![27, 34];
        assert_eq!(TwoSum::two_sum(price, target), result);
        let price: Vec<i32> = vec![14, 15, 16, 22, 53, 60];
        let target = 76;
        let result = vec![16, 60];
        assert_eq!(TwoSum::two_sum(price, target), result);
    }
}
