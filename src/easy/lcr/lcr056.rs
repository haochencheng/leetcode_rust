use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;

use crate::tree::tree_node::TreeNode;

struct Solution {}

impl Solution {
    fn find_target(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> bool {
        if root.is_none() {
            return false;
        }

        let mut hash_set = HashSet::new();
        Self::find(root, &mut hash_set, k)
    }

    fn find(root: Option<Rc<RefCell<TreeNode>>>, hash_set: &mut HashSet<i32>, k: i32) -> bool {
        if let Some(ref node) = root {
            if hash_set.contains(&(k - node.as_ref().borrow().val)) {
                return true;
            }
            // println!("root======={:#?}", node.as_ref().borrow());
            // println!("hashSet======={:#?}", hash_set);
            hash_set.insert(node.as_ref().borrow().val);
            return Self::find(node.as_ref().borrow().left.clone(), hash_set, k)
                || Self::find(node.as_ref().borrow().right.clone(), hash_set, k);
        }
        return false;
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn peak_index_in_mountain_array() {
        let root = TreeNode::build(vec![
            Some(8),
            Some(6),
            Some(10),
            Some(5),
            Some(7),
            Some(9),
            Some(11),
        ]);
        assert_eq!(Solution::find_target(root, 12), true);
    }
}
