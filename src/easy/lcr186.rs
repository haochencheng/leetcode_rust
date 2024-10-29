// LCR 186. 文物朝代判断
// 判断5个数是否连续 max-min<5

use std::collections::HashSet;

pub trait Solution {
    fn check_dynasty(places: Vec<i32>) -> bool;
}

pub struct CheckDynasty {}

impl Solution for CheckDynasty {
    fn check_dynasty(places: Vec<i32>) -> bool {
        let mut place_set = HashSet::new();
        let mut min = 14;
        let mut max = -1;
        for &place in &places {
            if place == 0 {
                continue;
            }
            if place_set.contains(&place) {
                return false;
            }
            place_set.insert(place);
            max = place.max(max);
            min = place.min(min);
        }
        return max - min < 5;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn judge_circle() {
        let places = vec![0, 6, 9, 0, 7];
        assert_eq!(CheckDynasty::check_dynasty(places), true);
    }
}
