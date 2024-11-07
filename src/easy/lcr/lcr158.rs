// LCR 158. 库存管理 II
// 摩尔投票法 假定肯定一个数出现次数大于数组的一半，这个数为num，其他数为other。
// 开始投票，遍历数组num+1，other-1.最后得出num。
pub trait Solution {
    fn inventory_management(stock: Vec<i32>) -> i32;
}

pub struct InventoryManagement {}

impl Solution for InventoryManagement {
    fn inventory_management(stock: Vec<i32>) -> i32 {
        let (mut ans, mut votes) = (0, 0);
        for &num in stock.iter() {
            if votes == 0 {
                ans = num;
            }
            if num == ans {
                votes += 1;
            } else {
                votes -= 1;
            }
        }
        return ans;
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn judge_circle() {
        let stock = vec![6, 1, 3, 1, 1, 1];
        assert_eq!(InventoryManagement::inventory_management(stock), 1);
    }
}
