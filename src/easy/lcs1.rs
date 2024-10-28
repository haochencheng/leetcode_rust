// lcs1.  下载插件
// 思路，每分钟翻倍就是log2(n),满足下载要求开始下载

pub trait Solution {
    fn least_minutes(n: i32) -> i32;
}


pub struct LeastMinutes {}

impl Solution for LeastMinutes {

    fn least_minutes(n: i32) -> i32 {
        if n == 1 {
           return  n;
        }
        let mut result = 0;
        let mut speed=1;
        while speed < n {
            speed = speed<<1;
            result+=1;
        }
        return result+1;
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn max_unique_split() {
        assert_eq!(LeastMinutes::least_minutes(2),2);
        assert_eq!(LeastMinutes::least_minutes(4),3);
        assert_eq!(LeastMinutes::least_minutes(5),4);
        assert_eq!(LeastMinutes::least_minutes(7),4);
    }
}
