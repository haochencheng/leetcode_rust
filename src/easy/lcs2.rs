// 599. 两个列表的最小索引总和
// 思路：hahsmap记录list1的值和索引，记录最小索引和，如果有新的跟新最小索引和数据

use std::collections::HashMap;

pub trait Solution {
    fn half_questions(questions: Vec<i32>) -> i32;
}

pub struct HalfQuestions {}

impl Solution for HalfQuestions {
    fn half_questions(questions: Vec<i32>) -> i32 {
        if questions.is_empty() {
            return 0;
        }
        let mut map = HashMap::new();
        questions
            .iter()
            .for_each(|&f| *map.entry(f).or_insert(0) += 1);
        let mut v: Vec<i32> = map.into_values().collect();
        v.sort_unstable();
        let mut x = 0;
        let mut n: i32 = questions.len() as i32 / 2;
        for i in v.iter().rev() {
            n -= i;
            x += 1;
            if n <= 0 {
                return x;
            }
        }
        x
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn max_unique_split() {
        let questions = vec![2, 1, 6, 2];
        let result = 1;
        assert_eq!(HalfQuestions::half_questions(questions), result);
    }
}
