use std::i32;
//
pub trait Solution {
    fn inventory_management(stock: Vec<i32>) -> i32;
}

pub struct InventoryManagement {}

impl Solution for InventoryManagement {
    fn inventory_management(stock: Vec<i32>) -> i32 {
        let mut min = i32::MAX;
        for i in 0..stock.len() {
            min = min.min(stock[i]);
        }
        return min;
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn judge_circle() {
        let arr: Vec<i32> = vec![4, 5, 8, 3, 4];
        let ans = 3;
        assert_eq!(InventoryManagement::inventory_management(arr), ans);
    }
}
