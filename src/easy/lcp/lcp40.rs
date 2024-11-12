use std::i32;

struct Solution {}

impl Solution {
    pub fn maximum_score(cards: Vec<i32>, cnt: i32) -> i32 {
        let mut cards = cards;
        cards.sort_by(|a, b| b.cmp(a));
        let mut ans = 0;
        let mut tmp = 0;
        let mut odd = -1;
        let mut even = -1;
        let cnt = cnt as usize;
        for i in 0..cnt {
            tmp += cards[i];
            if cards[i] % 2 == 0 {
                even = cards[i];
            }
            if cards[i] % 2 == 1 {
                odd = cards[i];
            }
        }
        if tmp % 2 == 0 {
            return tmp;
        }
        for i in cnt..cards.len() {
            if cards[i] % 2 == 1 {
                if even != -1 {
                    ans = ans.max(tmp - even + cards[i]);
                }
            } else {
                if odd != -1 {
                    ans = ans.max(tmp - odd + cards[i]);
                }
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn maximum_score() {
        let cards = vec![3, 3, 1];
        let cnt = 1;
        assert_eq!(Solution::maximum_score(cards, cnt), 0);
        let cards = vec![1, 2, 8, 9];
        let cnt = 3;
        assert_eq!(Solution::maximum_score(cards, cnt), 18);
    }
}
