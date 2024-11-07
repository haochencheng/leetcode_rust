use std::cell::RefCell;
use std::rc::Rc;

use crate::tree::tree_node::TreeNode;

pub struct Solution {}

impl Solution {
    pub fn increasing_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let new_root = Some(Rc::new(RefCell::new(TreeNode::new(0)))); // 哨兵节点
        let mut current = new_root.clone();

        Self::inorder(&root, &mut current);

        // new_root 的右子节点即为实际的树根
        new_root.unwrap().borrow_mut().right.take()
    }

    fn inorder(root: &Option<Rc<RefCell<TreeNode>>>, current: &mut Option<Rc<RefCell<TreeNode>>>) {
        if let Some(node) = root {
            // 递归遍历左子树
            Self::inorder(&node.borrow().left, current);

            // 通过先创建新节点来避免借用冲突
            let new_node = Rc::new(RefCell::new(TreeNode::new(node.borrow().val)));
            if let Some(cur) = current {
                cur.borrow_mut().right = Some(new_node.clone());
                *current = Some(new_node);
            }

            // 递归遍历右子树
            Self::inorder(&node.borrow().right, current);
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn increasing_bst() {
        let root = TreeNode::build(vec![
            Some(5),
            Some(3),
            Some(6),
            Some(2),
            Some(4),
            None,
            Some(8),
            Some(1),
            None,
            None,
            None,
            Some(7),
            Some(9),
        ]);
        let result = TreeNode::build(vec![
            Some(1),
            None,
            Some(2),
            None,
            Some(3),
            None,
            Some(4),
            None,
            Some(5),
            None,
            Some(6),
            None,
            Some(7),
            None,
            Some(8),
            None,
            Some(9),
        ]);
        let node = Solution::increasing_bst(root);
        println!("node===={:#?}", TreeNode::to_vec(node.clone()));
        assert_eq!(node, result)
    }
}
