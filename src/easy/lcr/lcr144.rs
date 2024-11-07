use crate::tree::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

// LCR 144. 翻转二叉树
pub trait Solution {
    fn flip_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>>;

    fn flip_tree_sub(root: Option<Rc<RefCell<TreeNode>>>);
}

pub struct FlipTree {}

impl Solution for FlipTree {
    fn flip_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::flip_tree_sub(root.clone());
        return root;
    }

    fn flip_tree_sub(root: Option<Rc<RefCell<TreeNode>>>) {
        if let Some(node) = root {
            let tmp = node.borrow().left.clone();
            let mut tmp_node = node.borrow_mut();
            tmp_node.left = tmp_node.right.clone();
            tmp_node.right = tmp;
            Self::flip_tree_sub(tmp_node.left.clone());
            Self::flip_tree_sub(tmp_node.right.clone());
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn judge_circle() {
        let root = vec![
            Some(5),
            Some(7),
            Some(9),
            Some(8),
            Some(3),
            Some(2),
            Some(4),
        ];
        let root = TreeNode::build(root);
        let reuslt = vec![
            Some(5),
            Some(9),
            Some(7),
            Some(4),
            Some(2),
            Some(3),
            Some(8),
        ];
        let ans = TreeNode::to_vec(FlipTree::flip_tree(root));
        println!("ans======{:#?}", ans);
        assert_eq!(ans, reuslt);
    }
}
