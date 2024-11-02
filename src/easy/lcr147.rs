use std::i32;

struct MinStack {
    val: Vec<i32>,
    min: i32,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {
    /** initialize your data structure here. */
    fn new() -> Self {
        MinStack {
            val: Vec::new(),
            min: i32::MAX,
        }
    }

    fn push(&mut self, x: i32) {
        &self.val.push(x);
        self.min = *&self.min.min(x);
    }

    fn pop(&mut self) {
        let p = &self.val.pop();
        if p.unwrap() == self.min {
            // 重新计算
            let mut min = i32::MAX;
            for ele in &self.val.clone() {
                min = min.min(*ele);
            }
            self.min = min;
        }
    }

    fn top(&self) -> i32 {
        if self.val.len() == 0 {
            return 0;
        }
        return *&self.val[(*&self.val.len() - 1) as usize];
    }

    fn get_min(&self) -> i32 {
        return self.min;
    }
}

/**
 * Your MinStack object will be instantiated and called as such:
 * let obj = MinStack::new();
 * obj.push(x);
 * obj.pop();
 * let ret_3: i32 = obj.top();
 * let ret_4: i32 = obj.get_min();
 */

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn min_stack() {
        let mut obj = MinStack::new();
        obj.push(-2);
        obj.push(2);
        obj.push(-3);
        println!("obj======{:#?}", obj.val);
        assert_eq!(obj.get_min(), -3);
        obj.pop();
        println!("obj======{:#?}", obj.val);
        assert_eq!(obj.top(), 2);
        println!("obj.get_min()======{:#?}", obj.get_min());
        assert_eq!(obj.get_min(), -2);
    }
}
