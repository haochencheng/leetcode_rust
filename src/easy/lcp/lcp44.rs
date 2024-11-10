use crate::tree::tree_node::TreeNode;
use std::cell::RefCell;
use std::collections::HashSet;
use std::i32;
use std::rc::Rc;

struct Solution {}

impl Solution {
    pub fn num_color(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return 0;
        }
        let mut hash_set = HashSet::new();
        Self::dps(&root, &mut hash_set);
        hash_set.len() as i32
    }

    fn dps(root: &Option<Rc<RefCell<TreeNode>>>, hash_set: &mut HashSet<i32>) {
        if let Some(ref node) = root {
            hash_set.insert(node.borrow().val);
            Self::dps(&node.borrow().left, hash_set);
            Self::dps(&node.borrow().right, hash_set);
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn num_color() {
        let root = TreeNode::build(vec![Some(1), Some(3), Some(2), Some(1), None, Some(2)]);
        assert_eq!(Solution::num_color(root), 3);
    }
}
