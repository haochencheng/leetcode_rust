// LCR 142. 训练计划 IV
pub trait Solution {
    fn training_plan(actions: Vec<i32>) -> Vec<i32>;
}

pub struct TrainningPlan {}

impl Solution for TrainningPlan {
    fn training_plan(actions: Vec<i32>) -> Vec<i32> {
        if actions.is_empty() {
            return Vec::new();
        }
        let mut arr = actions.clone();
        let mut pre = 0;
        for i in 0..arr.len() {
            if arr[i] % 2 != 0 {
                let tmp = arr[i];
                arr[i] = arr[pre];
                arr[pre] = tmp;
                pre = pre + 1;
            }
        }
        arr
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn judge_circle() {
        let arr: Vec<i32> = vec![1, 2, 3, 4, 5];
        let result: Vec<i32> = vec![1, 3, 5, 2, 4];
        let ans = TrainningPlan::training_plan(arr);
        assert_eq!(ans, result);
    }
}
