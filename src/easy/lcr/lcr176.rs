// LCR 176. 判断是否为平衡二叉树
use crate::tree::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub trait Solution {
    fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool;
}

pub struct IsBalanced {}

impl Solution for IsBalanced {
    fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if root.is_none() {
            return true;
        }
        fn check_is_balanced(node: &Option<Rc<RefCell<TreeNode>>>) -> (bool, i32) {
            if let Some(node) = node {
                let left = check_is_balanced(&node.borrow().left);
                let right = check_is_balanced(&node.borrow().right);
                let is_balanced = left.0 && right.0 && (left.1 - right.1).abs() <= 1;
                let high: i32 = 1 + left.1.max(right.1);
                (is_balanced, high)
            } else {
                (true, 0)
            }
        }
        check_is_balanced(&root).0
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::tree::tree_node::TreeNode;

    #[test]
    fn judge_circle() {
        let root = vec![
            Some(1),
            Some(2),
            Some(2),
            Some(3),
            Some(3),
            None,
            None,
            Some(4),
            Some(4),
        ];
        let root = TreeNode::build(root);
        println!("root ====={:#?}", root);
        assert_eq!(IsBalanced::is_balanced(root), false);
    }
}
