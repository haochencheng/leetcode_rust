use std::collections::VecDeque;

struct MovingAverage {
    k: i32,
    deque: VecDeque<i32>,
    sum: i32,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MovingAverage {
    /** Initialize your data structure here. */
    fn new(size: i32) -> Self {
        MovingAverage {
            k: size,
            deque: VecDeque::new(),
            sum: 0,
        }
    }
    // 1 2 3 4 5 -> 3
    // 1
    // 1 3
    // 1 3 6
    fn next(&mut self, val: i32) -> f64 {
        self.deque.push_back(val);
        self.sum += val;
        if self.deque.len() > self.k as usize {
            if let Some(p) = self.deque.pop_front() {
                self.sum -= p;
            }
        }
        return self.sum as f64 / (self.deque.len() as f64);
    }
}

/**
 * Your MovingAverage object will be instantiated and called as such:
 * let obj = MovingAverage::new(size);
 * let ret_1: f64 = obj.next(val);
 */

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn moving_average() {
        let mut obj = MovingAverage::new(3);
        assert_eq!(obj.next(1), 1.0);
        assert_eq!(obj.next(10), 5.5);
        assert_eq!(obj.next(3), 4.666666666666667);
        assert_eq!(obj.next(5), 6.0);
    }
}
