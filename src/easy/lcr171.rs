// LCR 171. 训练计划 V
use crate::tree::list_node::ListNode;
use std::borrow::{Borrow, BorrowMut};
use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;

pub trait Solution {
    fn intersection_node(
        head_a: Option<Rc<RefCell<ListNode>>>,
        head_b: Option<Rc<RefCell<ListNode>>>,
    ) -> i32;

    fn print_val(tmp: &str, node: &Option<Rc<RefCell<ListNode>>>);
}

pub struct IntersectionNode {}

impl Solution for IntersectionNode {
    fn intersection_node(
        head_a: Option<Rc<RefCell<ListNode>>>,
        head_b: Option<Rc<RefCell<ListNode>>>,
    ) -> i32 {
        if head_a.is_none() || head_b.is_none() {
            return 0;
        }

        let mut ptr_a = head_a.clone();
        let mut ptr_b = head_b.clone();

        while &ptr_a != &ptr_b {
            ptr_a = match ptr_a {
                Some(node) => node.as_ref().borrow().next.clone(),
                None => head_b.clone(), // Switch to head_b after reaching the end of list A
            };

            ptr_b = match ptr_b {
                Some(node) => node.as_ref().borrow().next.clone(),
                None => head_a.clone(), // Switch to head_a after reaching the end of list B
            };
            Self::print_val(&"a", &ptr_a);
            Self::print_val(&"b", &ptr_b);
        }

        ptr_a.map(|node| node.as_ref().borrow().val).unwrap() // ptr_a and ptr_b will be the same when they meet
    }

    fn print_val(tmp: &str, node: &Option<Rc<RefCell<ListNode>>>) {
        if let Some(rc_node) = node {
            // 解包 Rc
            let borrowed_node = rc_node.as_ref().borrow();
            // 打印 val 值
            println!("Value: {}======,{}", tmp, borrowed_node.val);
        } else {
            println!("Node {}====== is None", tmp);
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn judge_circle() {
        let intersection_a = vec![4, 1];
        let intersection_b = vec![5, 0, 1];
        let intersection_c = vec![8, 4, 5];

        let head_c = ListNode::build(intersection_c.clone());
        let head_a = ListNode::build_with_list_node(intersection_a.clone(), &head_c);
        let head_b = ListNode::build_with_list_node(intersection_b.clone(), &head_c);

        let node = ListNode {
            val: intersection_a[0],
            next: head_a.clone().unwrap().as_ref().borrow().next.clone(),
        };
        println!(" intersection_a ====  {}", node);
        let node_b = ListNode {
            val: intersection_b[0],
            next: head_b.clone().unwrap().as_ref().borrow().next.clone(),
        };
        println!(" intersection_b ====  {}", node_b);
        assert_eq!(IntersectionNode::intersection_node(head_a, head_b), 8);
    }
}
