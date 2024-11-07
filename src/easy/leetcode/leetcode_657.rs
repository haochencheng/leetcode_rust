// 657. 机器人能否返回原点

pub trait Solution {
    fn judge_circle(moves: String) -> bool;
}

pub struct JudgeCircle {}

impl Solution for JudgeCircle {
    fn judge_circle(moves: String) -> bool {
        let mut x = 0;
        let mut y = 0;
        for _str in moves.bytes() {
            if b'U' == _str {
                y = y + 1
            } else if b'D' == _str {
                y = y - 1
            } else if b'L' == _str {
                x = x - 1;
            } else {
                x = x + 1;
            }
        }
        return x == y && x == 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn judge_circle() {
        let moves = "UD".to_string();
        assert_eq!(JudgeCircle::judge_circle(moves), true);
    }
}
