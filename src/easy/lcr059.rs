use std::{cmp::Reverse, collections::BinaryHeap, i32};

// //
// pub trait Solution {
//     fn find_maximum_xor(nums: Vec<i32>) -> i32;
// }

// 2, 4, 5, 8
// add 3 = 2,3,4,5,8 => 4
// add 5 = 2,3,4,5,5,8 => 5
pub struct KthLargest {
    queue: BinaryHeap<Reverse<i32>>,
    k: i32,
}

impl KthLargest {
    fn new(k: i32, nums: Vec<i32>) -> Self {
        let queue = BinaryHeap::new();
        let mut ans = KthLargest { k: k, queue: queue };
        for ele in nums {
            ans.add(ele);
        }
        ans
    }

    fn add(&mut self, val: i32) -> i32 {
        self.queue.push(Reverse(val));
        println!("queue:{:#?}", self.queue);
        if self.queue.len() > self.k.try_into().unwrap() {
            self.queue.pop();
            println!("queue pop:{:#?}", self.queue);
        }
        if let Some(&Reverse(v)) = self.queue.peek() {
            return v;
        }
        return -1;
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn peak_index_in_mountain_array() {
        let mut kth_largest = KthLargest::new(3, vec![4, 5, 8, 2]);
        assert_eq!(kth_largest.add(3), 4);
        assert_eq!(kth_largest.add(5), 5);
        assert_eq!(kth_largest.add(10), 5);
        assert_eq!(kth_largest.add(9), 8);
        assert_eq!(kth_largest.add(4), 8);
        // let mut kth_largest = KthLargest::new(1, vec![]);
        // assert_eq!(kth_largest.add(-3), -3);
        // assert_eq!(kth_largest.add(-2), -2);
        // assert_eq!(kth_largest.add(-4), -2);
        // assert_eq!(kth_largest.add(0), 0);
        // assert_eq!(kth_largest.add(4), 4);
    }
}
