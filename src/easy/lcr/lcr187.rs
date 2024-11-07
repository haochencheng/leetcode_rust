// LCR 187. 破冰游戏
// 建模为函数 f(num,target)
// num 会踢出 target%num
// 0123456 踢出3 => 012456 从4开始计数
// 012456 踢出0 => 12456 从1开始计数
// 12456 踢出5 => 6124 从6开始计数
// 6124 踢出4 => 126 从6开始计数
// 126 踢出6 =>12 从1开始计数
// 12 踢出2 => 1 game over
// f(1) = 1
// f(2)=(f(1)+target)%2
// f(3)=(f(2)+target)%3
// f(n)=(f(n-1)+target)&n

pub trait Solution {
    fn ice_breaking_game(num: i32, target: i32) -> i32;
}

pub struct IceBreakingFame {}

impl Solution for IceBreakingFame {
    fn ice_breaking_game(num: i32, target: i32) -> i32 {
        println!("f(2)===={}", (0 + 4) % 2);
        println!("f(3)===={}", (2 + 4) % 3);
        println!("f(4)===={}", (4 + (2 + 4)) % 4);
        let mut last = 0;
        for i in 2..num + 1 {
            last = (last + target) % i;
        }
        return last;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn judge_circle() {
        let num = 7;
        let target = 4;
        assert_eq!(IceBreakingFame::ice_breaking_game(num, target), 1);
    }
}
