use std::cell::RefCell;
use std::rc::Rc;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }

    pub fn build(values: Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
        if values.is_empty() {
            return None;
        }
        let root = Rc::new(RefCell::new(TreeNode::new(values[0].unwrap())));
        let mut queue = vec![Rc::clone(&root)];
        let mut i = 1;
        while i < values.len() {
            let current = queue.remove(0);
            if let Some(val) = values[i] {
                let left_node = Rc::new(RefCell::new(TreeNode::new(val)));
                current.borrow_mut().left = Some(Rc::clone(&left_node));
                queue.push(left_node);
            }
            i += 1;
            if i < values.len() {
                if let Some(val) = values[i] {
                    let rifht_node = Rc::new(RefCell::new(TreeNode::new(val)));
                    current.borrow_mut().right = Some(Rc::clone(&rifht_node));
                    queue.push(rifht_node);
                }
                i += 1;
            }
        }
        Some(root)
    }
}
