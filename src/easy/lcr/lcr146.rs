use std::i32;

// LCR 146. 螺旋遍历二维数组
pub trait Solution {
    fn spiral_array(array: Vec<Vec<i32>>) -> Vec<i32>;
}

pub struct SpiralArray {}

impl Solution for SpiralArray {
    fn spiral_array(array: Vec<Vec<i32>>) -> Vec<i32> {
        if array.is_empty() {
            return Vec::new();
        }
        // 1 2 3
        // 8 9 4
        // 7 6 5
        // 打印顺序 从左到右，从上到下，从右到左，从下到上
        let mut ans = Vec::new();
        let (mut l, mut r, mut t, mut b) = (0, array[0].len() - 1, 0, array.len() - 1);
        loop {
            // 向右
            // println!("t=={},b==={},l==={},r==={}", t, b, l, r);
            for i in l..(r + 1) {
                ans.push(array[t][i]);
            }
            t = t + 1;
            if t > b {
                break;
            }
            // 向下
            // println!("t=={},b==={},l==={},r==={}", t, b, l, r);
            for i in t..(b + 1) {
                ans.push(array[i][r]);
            }
            if r == 0 {
                break;
            }
            r = r - 1;
            if l > r {
                break;
            }
            // println!("t=={},b==={},l==={},r==={}", t, b, l, r);
            // 向左
            for i in (l..r + 1).rev() {
                ans.push(array[b][i]);
            }
            if b == 0 {
                break;
            }
            b = b - 1;
            if t > b {
                break;
            }
            // println!("t=={},b==={},l==={},r==={}", t, b, l, r);
            // 向上
            for i in (t..b + 1).rev() {
                ans.push(array[i][l]);
            }
            l = l + 1;
            if l > r {
                break;
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn spiral_array() {
        let array = vec![vec![2, 3]];
        let result = vec![2, 3];
        assert_eq!(SpiralArray::spiral_array(array), result);
        let array = vec![vec![1, 2, 3], vec![8, 9, 4], vec![7, 6, 5]];
        let result = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        assert_eq!(SpiralArray::spiral_array(array), result);
        let array = vec![vec![7], vec![9], vec![6]];
        let result = vec![7, 9, 6];
        assert_eq!(SpiralArray::spiral_array(array), result);
    }
}
