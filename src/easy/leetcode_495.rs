// 509. 斐波那契数

pub trait Solution {
    fn find_poisoned_duration(time_series: Vec<i32>, duration: i32) -> i32  ;
}


pub struct FindPoisonedDuration {}

impl Solution for FindPoisonedDuration {

    fn find_poisoned_duration(time_series: Vec<i32>, duration: i32) -> i32  {
        if time_series.is_empty(){
            return 0;
        }
        let mut result = 0;
        let mut expire = 0;
        for i in 0..time_series.len()  {
            if time_series[i]>expire {
                result+=duration;
            }else {
                result+=time_series[i]+duration-expire;
            }
            expire=time_series[i]+duration;
        }
        return result;

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn max_unique_split() {
        let time_series = vec![1,4];
        let duration=2;
        assert_eq!(FindPoisonedDuration::find_poisoned_duration(time_series,duration),3);
    }
}
