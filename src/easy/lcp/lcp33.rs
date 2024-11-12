use std::i32;

struct Solution {}

impl Solution {
    pub fn store_water(bucket: Vec<i32>, vat: Vec<i32>) -> i32 {
        let n = bucket.len();

        let &max_pull = vat.iter().max().unwrap();

        (1..=max_pull)
            .map(|pull| {
                (0..n).fold(pull, |t, i| {
                    let least = (vat[i] as f64 / pull as f64).ceil() as i32;
                    t + std::cmp::max(0, least - bucket[i])
                })
            })
            .min()
            .unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn maximum_score() {
        let bucket = vec![1, 3];
        let vat = vec![6, 8];
        assert_eq!(Solution::store_water(bucket, vat), 4);
        let bucket = vec![9, 0, 1];
        let vat = vec![0, 2, 2];
        assert_eq!(Solution::store_water(bucket, vat), 3);
    }
}
