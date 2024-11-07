// 3226. 使两个整数相等的位更改次数
pub trait Solution {
    fn min_changes(n: i32, k: i32) -> i32;

    fn binary_array(n: i32) -> Vec<i32>;
}

pub struct MinChanges {}

// LCR 169. 招式拆解 II
impl Solution for MinChanges {
    fn min_changes(n: i32, k: i32) -> i32 {
        // 1110
        // 1101
        // no
        // 1110
        // 0100
        // yes

        if n == k {
            return 0;
        }
        let n_bytes: Vec<i32> = Self::binary_array(n);
        let k_bytes = Self::binary_array(k);
        // println!("n_bytes====={:#?}", n_bytes);
        // println!("k_bytes====={:#?}", k_bytes);
        if k_bytes.len() > n_bytes.len() {
            return -1;
        }
        let sub = n_bytes.len() - k_bytes.len();
        println!("sub ====={}", sub);
        let mut ans = 0;
        for i in (0..=(n_bytes.len() - 1)).rev() {
            let j: i32 = i as i32 - sub as i32;
            // println!("i===={},j===={}", i, j);
            if j < 0 {
                if n_bytes[i] == 1 {
                    ans += 1;
                }
                continue;
            }
            if n_bytes[i] == 0 && k_bytes[j as usize] == 1 {
                return -1;
            } else if n_bytes[i] == 1 && k_bytes[j as usize] == 0 {
                ans += 1;
            }
        }
        return ans;
    }

    fn binary_array(n: i32) -> Vec<i32> {
        format!("{:b}", n)
            .chars() // Convert binary string to characters
            .map(|c| c.to_digit(2).unwrap() as i32) // Convert '0' or '1' characters to integers
            .collect() // Collect into a vector
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn judge_circle() {
        let n = 10;
        let binary_representation = MinChanges::binary_array(n);
        println!("{:?}", binary_representation); // Output: [1, 0, 1, 0]
        assert_eq!(MinChanges::min_changes(13, 4), 2);
        assert_eq!(MinChanges::min_changes(14, 13), -1);
    }
}
