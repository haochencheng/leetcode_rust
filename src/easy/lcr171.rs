// LCR 171. 训练计划 V
use crate::tree::list_node::ListNode;
use std::borrow::{Borrow, BorrowMut};
use std::cell::RefCell;
use std::rc::Rc;

pub trait Solution {
    fn intersection_node(
        head_a: Option<Rc<RefCell<ListNode>>>,
        head_b: Option<Rc<RefCell<ListNode>>>,
    ) -> i32;
}

pub struct IntersectionNode {}

impl Solution for IntersectionNode {
    fn intersection_node(
        head_a: Option<Rc<RefCell<ListNode>>>,
        head_b: Option<Rc<RefCell<ListNode>>>,
    ) -> i32 {
        let mut a = head_a.clone();
        let mut b = head_b.clone();
        if let Some(node) = a {}
        return 0;
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn judge_circle() {
        let intersection_a = vec![4, 1, 8, 4, 5];
        let intersection_b = vec![5, 0, 1, 8, 4, 5];
        let head_a = ListNode::build(intersection_a.clone());
        let head_b = ListNode::build(intersection_b);
        let node = ListNode {
            val: intersection_a[0],
            next: head_a.clone().unwrap().borrow_mut().next.clone(),
        };
        println!(" intersection_a ====  {}", node);
        assert_eq!(IntersectionNode::intersection_node(head_a, head_b), 3);
    }
}
