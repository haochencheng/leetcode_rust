use crate::tree::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

// LCR 145. 判断对称二叉树
pub trait Solution {
    fn check_symmetric_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool;

    fn check_symmetric(
        root: Option<Rc<RefCell<TreeNode>>>,
        right: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool;
}

pub struct CheckSymmetricTree {}

impl Solution for CheckSymmetricTree {
    fn check_symmetric_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if root.is_none() {
            return true;
        }
        return Self::check_symmetric(root.clone(), root.clone());
    }

    fn check_symmetric(
        left: Option<Rc<RefCell<TreeNode>>>,
        right: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        if left.is_none() && right.is_none() {
            return true;
        }
        if left.is_none() || right.is_none() {
            return false;
        }
        let right_unwrap = right.unwrap();
        let right_binding = right_unwrap.as_ref().borrow();
        let left_unwrap = left.unwrap();
        let left_binging = left_unwrap.as_ref().borrow();
        if left_binging.val != right_binding.val {
            return false;
        }
        return Self::check_symmetric(left_binging.left.clone(), right_binding.right.clone())
            && Self::check_symmetric(left_binging.right.clone(), right_binding.left.clone());
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn judge_circle() {
        let root = vec![
            Some(6),
            Some(7),
            Some(7),
            Some(8),
            Some(9),
            Some(9),
            Some(8),
        ];
        let root = TreeNode::build(root);
        assert_eq!(CheckSymmetricTree::check_symmetric_tree(root), true);

        let root = vec![Some(1), Some(2), Some(2), None, Some(3), None, Some(3)];
        let root = TreeNode::build(root);
        assert_eq!(CheckSymmetricTree::check_symmetric_tree(root), false);

        let root = vec![Some(1), Some(2), Some(3)];
        let root = TreeNode::build(root);
        assert_eq!(
            CheckSymmetricTree::check_symmetric_tree(root.clone()),
            false
        );
    }
}
