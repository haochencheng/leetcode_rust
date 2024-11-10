use std::i32;

struct Solution {}

impl Solution {
    pub fn give_gem(mut gem: Vec<i32>, operations: Vec<Vec<i32>>) -> i32 {
        for arr in operations {
            let left = arr[0] as usize;
            let right = arr[1] as usize;
            let give = gem[left] / 2;
            gem[left] = gem[left] - give;
            gem[right] = gem[right] + give;
        }
        let mut max = i32::MIN;
        let mut min = i32::MAX;
        for ele in gem {
            max = max.max(ele);
            min = min.min(ele);
        }
        max - min
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn give_gem() {
        let gem = vec![3, 1, 2];
        let operations = vec![vec![0, 2], vec![2, 1], vec![2, 0]];
        assert_eq!(Solution::give_gem(gem, operations), 2);
    }
}
