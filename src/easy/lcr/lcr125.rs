use std::i32;
// //
pub trait Solution {
    fn new() -> Self;

    fn append_tail(&mut self, value: i32);

    fn delete_head(&mut self) -> i32;
}

pub struct CQueue {
    val: Vec<i32>,
}

impl Solution for CQueue {
    fn new() -> Self {
        CQueue { val: Vec::new() }
    }

    fn append_tail(&mut self, value: i32) {
        self.val.push(value);
    }

    fn delete_head(&mut self) -> i32 {
        if self.val.is_empty() {
            -1 // Assuming -1 indicates an empty queue
        } else {
            self.val.remove(0)
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn judge_circle() {
        let mut obj = CQueue::new();
        obj.append_tail(1);
        let _ret_2: i32 = obj.delete_head();
    }
}
