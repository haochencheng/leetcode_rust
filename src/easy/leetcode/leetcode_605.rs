// 605. 种花问题

pub trait Solution {
    fn can_place_flowers(flowerbed: Vec<i32>, n: i32) -> bool;
}

pub struct CanPlaceFlowers {}

impl Solution for CanPlaceFlowers {
    fn can_place_flowers(flowerbed: Vec<i32>, n: i32) -> bool {
        if flowerbed.is_empty() {
            return n == 0;
        }
        let mut next_flower = 0;
        let mut result = n;
        let mut flowerbed = flowerbed;
        for i in 0..flowerbed.len() {
            if result == 0 {
                return true;
            }
            if flowerbed[i] == 0 {
                if i == flowerbed.len() - 1 {
                    if next_flower <= i {
                        result = result - 1;
                    }
                    flowerbed[i] = 1;
                } else {
                    if flowerbed[i + 1] == 0 && next_flower <= i {
                        result = result - 1;
                        flowerbed[i] = 1;
                        next_flower = i + 2;
                    }
                }
            } else {
                next_flower = i + 2;
            }
        }
        return result == 0;
    }
}

#[cfg(test)]
mod tests {

    use std::vec;

    use super::*;

    #[test]
    fn max_unique_split() {
        let flowerbed = vec![1, 0, 0, 0, 1];
        let n = 1;
        assert_eq!(CanPlaceFlowers::can_place_flowers(flowerbed, n), true);
        let flowerbed = vec![1, 0, 0, 0, 1];
        let n = 2;
        assert_eq!(CanPlaceFlowers::can_place_flowers(flowerbed, n), false);
        let flowerbed = vec![1, 0, 0, 0, 1, 0, 0];
        let n = 2;
        assert_eq!(CanPlaceFlowers::can_place_flowers(flowerbed, n), true);
    }
}
