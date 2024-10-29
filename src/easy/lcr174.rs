// LCR 176. 判断是否为平衡二叉树
use crate::tree::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub trait Solution {
    fn find_target_node(root: Option<Rc<RefCell<TreeNode>>>, cnt: i32) -> i32;
}

pub struct FindTargetNode {}

impl Solution for FindTargetNode {
    fn find_target_node(root: Option<Rc<RefCell<TreeNode>>>, cnt: i32) -> i32 {
        if root.is_none() {
            return 0;
        }
        fn scan_tree_node(
            node: &Option<Rc<RefCell<TreeNode>>>,
            cnt: i32,
            arr: &mut Vec<i32>,
        ) -> () {
            if let Some(node) = node {
                let val = &node.borrow().val;
                arr.push(*val);
                scan_tree_node(&node.borrow().right, cnt, arr);
                scan_tree_node(&node.borrow().left, cnt, arr);
                if arr.len() > cnt as usize {
                    return;
                }
            }
        }
        let mut arr = Vec::new();
        scan_tree_node(&root, cnt, &mut arr);
        arr.sort();
        // arr.reverse();
        // println!("arr===={:#?}", arr);
        arr[arr.len() - cnt as usize]
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::tree::tree_node::TreeNode;

    #[test]
    fn judge_circle() {
        let root = vec![Some(7), Some(3), Some(9), Some(1), Some(5)];
        let root = TreeNode::build(root);
        // println!("root ====={:#?}",root);
        assert_eq!(FindTargetNode::find_target_node(root, 2), 7);
        let root = vec![
            Some(10),
            Some(5),
            Some(15),
            Some(2),
            Some(7),
            None,
            Some(20),
            Some(1),
            None,
            Some(6),
            Some(8),
        ];
        let root = TreeNode::build(root);
        // println!("root ====={:#?}",root);
        assert_eq!(FindTargetNode::find_target_node(root, 4), 8);
    }
}
