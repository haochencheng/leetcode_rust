use std::collections::VecDeque;

pub struct RecentCounter {
    window: VecDeque<i32>,
}

impl RecentCounter {
    fn new() -> Self {
        RecentCounter {
            window: VecDeque::new(),
        }
    }

    fn ping(&mut self, t: i32) -> i32 {
        self.window.push_back(t);
        while !self.window.is_empty() && self.window.front().unwrap() < &(t - 3000) {
            self.window.pop_front();
        }
        println!("deque :{:#?}", self.window);
        self.window.len().try_into().unwrap()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    /**
     * Your RecentCounter object will be instantiated and called as such:
     * let obj = RecentCounter::new();
     * let ret_1: i32 = obj.ping(t);
     * [3001,3100,6001,6002]
     * [1..3001]
     * [100..3100]
     */
    #[test]
    fn increasing_bst() {
        let mut obj = RecentCounter::new();
        assert_eq!(obj.ping(1), 1);
        assert_eq!(obj.ping(100), 2);
        assert_eq!(obj.ping(3001), 3);
        assert_eq!(obj.ping(3002), 3);
    }
}
