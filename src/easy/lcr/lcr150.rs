use crate::tree::tree_node::TreeNode;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

// LCR 150. 彩灯装饰记录 II
// 按层输出二叉树
pub trait Solution {
    fn decorate_record(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>>;

    fn decorate_record_sub(
        root: Option<Rc<RefCell<TreeNode>>>,
        level: &mut i32,
        ans: &mut HashMap<i32, Vec<i32>>,
    );
}

pub struct DecorateRecord {}

// LCR 169. 招式拆解 II
impl Solution for DecorateRecord {
    fn decorate_record(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        if root.is_none() {
            return vec![];
        }
        let mut level = 0;
        let mut ans: HashMap<i32, Vec<i32>> = HashMap::new();
        Self::decorate_record_sub(root, &mut level, &mut ans);
        // println!("ans==={:#?}", ans);
        let mut result = Vec::new();
        for i in 0..ans.iter().len() {
            if let Some(arr) = ans.get(&(i as i32)) {
                result.push(arr.clone());
            }
        }
        return result;
    }

    fn decorate_record_sub(
        root: Option<Rc<RefCell<TreeNode>>>,
        level: &mut i32,
        ans: &mut HashMap<i32, Vec<i32>>,
    ) {
        if let Some(ref node) = root {
            let borrow = node.as_ref().borrow();
            ans.entry(*level).or_insert_with(Vec::new).push(borrow.val);
            Self::decorate_record_sub(borrow.left.clone(), &mut (*level + 1), ans);
            Self::decorate_record_sub(borrow.right.clone(), &mut (*level + 1), ans);
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn judge_circle() {
        let root = vec![Some(8), Some(17), Some(21), Some(18), None, None, Some(6)];
        let root = TreeNode::build(root);
        let ans = vec![vec![8], vec![17, 21], vec![18, 6]];
        assert_eq!(DecorateRecord::decorate_record(root), ans);
    }
}
