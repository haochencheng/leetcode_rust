// LCR 172. 统计目标成绩的出现次数

pub trait Solution {
    fn count_target(scores: Vec<i32>, target: i32) -> i32;
}

pub struct CountTarget {}

impl Solution for CountTarget {
    fn count_target(scores: Vec<i32>, target: i32) -> i32 {
        if scores.is_empty() {
            return 0;
        }
        if scores[scores.len() - 1] < target {
            return 0;
        }
        if scores[0] > target {
            return 0;
        }
        if scores.len() == 1 {
            if scores[0] == target {
                return 1;
            } else {
                return 0;
            }
        }
        let (l, mut r, mut res) = (0, scores.len() - 1, 0);
        let mut pre = r;
        while l < r {
            if scores[r] > target {
                pre = r;
                r = r / 2;
            } else {
                break;
            }
        }
        for i in l..pre + 1 {
            if target == scores[i] {
                res += 1;
            }
        }
        return res;
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn judge_circle() {
        let scores = vec![2, 2, 3, 4, 4, 4, 5, 6, 6, 8];
        assert_eq!(CountTarget::count_target(scores, 4), 3);
        let scores = vec![1, 2, 3, 5, 7, 9];
        assert_eq!(CountTarget::count_target(scores, 6), 0);
    }
}
