use crate::tree::tree_node::TreeNode;
use std::cell::RefCell;
use std::i32;
use std::rc::Rc;

// //
pub trait Solution {
    fn deduce_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>>;
}

pub struct DeduceTree {}

impl Solution for DeduceTree {
    fn deduce_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        fn build(preorder: &[i32], inorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
            if preorder.is_empty() {
                return None;
            }
            let mut root = TreeNode {
                val: preorder[0],
                left: None,
                right: None,
            };
            let idx = inorder.iter().position(|x| x == &preorder[0]).unwrap();
            root.left = build(&preorder[1..idx + 1], &inorder[0..idx]);
            root.right = build(
                &preorder[idx + 1..preorder.len()],
                &inorder[idx + 1..inorder.len()],
            );
            Some(Rc::new(RefCell::new(root)))
        }
        build(&preorder, &inorder)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn judge_circle() {
        // let preorder = vec![3, 9, 20, 15, 7];
        // let inorder = vec![9, 3, 15, 20, 7];
        // let result = TreeNode::build(vec![
        //     Some(3),
        //     Some(9),
        //     Some(20),
        //     None,
        //     None,
        //     Some(15),
        //     Some(7),
        // ]);
        // assert_eq!(DeduceTree::deduce_tree(preorder, inorder), result);
        // let preorder = vec![1, 2];
        // let inorder = vec![2, 1];
        // let result = TreeNode::build(vec![Some(1), Some(2)]);
        // assert_eq!(DeduceTree::deduce_tree(preorder, inorder), result);
        // let preorder = vec![1, 2];
        // let inorder = vec![1, 2];
        // let result = TreeNode::build(vec![Some(1), None, Some(2)]);
        // assert_eq!(DeduceTree::deduce_tree(preorder, inorder), result);
        let preorder = vec![1, 2, 3];
        let inorder = vec![3, 2, 1];
        let result = TreeNode::build(vec![Some(1), Some(2), None, Some(3)]);
        assert_eq!(DeduceTree::deduce_tree(preorder, inorder), result);
    }
}
