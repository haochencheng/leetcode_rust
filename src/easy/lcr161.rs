pub trait Solution {
    fn max_sales(sales: Vec<i32>) -> i32;
}

pub struct MaxSales {}

// LCR 169. 招式拆解 II
impl Solution for MaxSales {
    fn max_sales(sales: Vec<i32>) -> i32 {
        let mut pre = 0;
        let mut ans = sales[0];
        for sale in sales {
            pre = sale.max(pre + sale);
            ans = ans.max(pre)
        }
        return ans;
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn judge_circle() {
        let sales = vec![-2, 1, -3, 4, -1, 2, 1, -5, 4];
        assert_eq!(MaxSales::max_sales(sales), 6);
    }
}
