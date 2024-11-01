use std::cell::RefCell;
use std::fmt;
use std::fmt::Display;
use std::rc::Rc;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Rc<RefCell<ListNode>>>,
}

impl Display for ListNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(node) = &self.next {
            return node.borrow().display(f);
        }
        write!(f, "None")
    }
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }

    pub fn build(values: Vec<i32>) -> Option<Rc<RefCell<ListNode>>> {
        if values.is_empty() {
            return None;
        }
        let root = Rc::new(RefCell::new(ListNode::new(values[0])));
        let mut queue = vec![Rc::clone(&root)];
        for i in 1..values.len() {
            let next = Rc::new(RefCell::new(ListNode::new(values[i])));
            let cur: Rc<RefCell<ListNode>> = queue.remove(0);
            cur.borrow_mut().next = Some(Rc::clone(&next));
            queue.push(next);
        }
        Some(root)
    }

    fn display(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} => ", &self.val)?;
        if let Some(node) = &self.next {
            return node.borrow().display(f);
        }
        write!(f, "None")
    }
}
