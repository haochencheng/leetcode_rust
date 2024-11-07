use std::cmp::Reverse;
use std::fmt::{self, Debug, Display};

// 定义一个 BinaryHeapQueue 结构体，用于封装基础的二叉堆操作
#[derive(Default)]
pub struct BinaryHeapQueue<T> {
    heap: Vec<T>,
}

impl<T: Ord + std::fmt::Debug> BinaryHeapQueue<T> {
    pub fn new() -> Self {
        BinaryHeapQueue { heap: Vec::new() }
    }

    pub fn push(&mut self, val: T) {
        self.heap.push(val);
        self.self_up(self.heap.len() - 1);
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.heap.is_empty() {
            return None;
        }
        // println!("queue===:{:#?}", self.heap);
        let last_index = self.heap.len() - 1;
        self.heap.swap(0, last_index);
        // println!("queue pop===:{:#?}", self.heap);
        let result = self.heap.pop();
        // if let Some(ref val) = &result {
        //     println!("pop======{:#?}", val);
        // }
        self.self_down(0);
        result
    }

    pub fn peek(&self) -> Option<&T> {
        self.heap.get(0)
    }

    fn self_up(&mut self, mut idx: usize) {
        while idx > 0 {
            let parent = (idx - 1) / 2;
            if self.heap[idx] <= self.heap[parent] {
                break;
            }
            self.heap.swap(parent, idx);
            idx = parent;
        }
    }

    fn self_down(&mut self, mut idx: usize) {
        let len = self.heap.len();
        while idx < len {
            let left = idx * 2 + 1;
            let right = idx * 2 + 2;
            let mut smallest = left;
            if right < len && self.heap[left] < self.heap[right] {
                smallest = right;
            }
            if smallest >= len || self.heap[idx] >= self.heap[smallest] {
                break;
            }
            self.heap.swap(idx, smallest);
            idx = smallest;
        }
    }
}

// 实现 fmt::Debug trait
impl<T: Debug> fmt::Debug for BinaryHeapQueue<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // 输出队列的二叉堆内容
        f.debug_list().entries(&self.heap).finish()
    }
}

// 实现 fmt::Display trait
impl<T: Display> fmt::Display for BinaryHeapQueue<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // 输出队列的元素列表，以易读的格式
        write!(f, "[")?;
        let mut first = true;
        for item in &self.heap {
            if !first {
                write!(f, ", ")?;
            }
            write!(f, "{}", item)?;
            first = false;
        }
        write!(f, "]")
    }
}

// 定义一个 KthLargest 结构体，用于计算第 K 大的元素
pub struct KthLargest {
    queue: BinaryHeapQueue<Reverse<i32>>,
    k: i32,
}

impl KthLargest {
    pub fn new(k: i32, nums: Vec<i32>) -> Self {
        let mut kth = KthLargest {
            queue: BinaryHeapQueue::new(),
            k,
        };
        for num in nums {
            kth.add(num);
        }
        kth
    }

    pub fn add(&mut self, val: i32) -> i32 {
        self.queue.push(Reverse(val));
        // println!("queue===:{:#?}", self.queue.heap);
        if self.queue.heap.len() > self.k as usize {
            self.queue.pop();
            // println!("queue=== pop:{:#?}", self.queue.heap);
        }
        match self.queue.peek() {
            Some(&Reverse(v)) => {
                // println!(" v=========={}", v);
                v
            }
            None => -1,
        }
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
        let mut kth_largest = KthLargest::new(1, vec![]);
        assert_eq!(kth_largest.add(-3), -3);
        assert_eq!(kth_largest.add(-2), -2);
        assert_eq!(kth_largest.add(-4), -2);
        assert_eq!(kth_largest.add(0), 0);
        assert_eq!(kth_largest.add(4), 4);
        let mut kth_largest = KthLargest::new(3, vec![5, 5, 8]);
        assert_eq!(kth_largest.add(10), 5);
    }
}
