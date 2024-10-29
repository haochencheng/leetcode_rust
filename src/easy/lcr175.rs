// LCR 176. 判断是否为平衡二叉树
use std::rc::Rc;
use std::cell::RefCell;
use crate::tree::tree_node::TreeNode;

pub trait Solution {
    fn calculate_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32;

}


pub struct CalculateDepth {}

impl Solution for CalculateDepth {

    fn calculate_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none(){
            return 0;
        } 
        fn check_is_balanced(node: &Option<Rc<RefCell<TreeNode>>>) -> (bool,i32) {
        if let Some(node) = node {
            let left = check_is_balanced(&node.borrow().left);
            let right = check_is_balanced(&node.borrow().right);
            let is_balanced = left.0 && right.0 && (left.1-right.1).abs()<=1;
            let high: i32 = 1+left.1.max(right.1);
            (is_balanced,high)
            } else {
                (true,0)
            }
            
        }
       check_is_balanced(&root).1
    }

   
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::tree::tree_node::TreeNode;

    #[test]
    fn judge_circle() {
        let root  = vec![Some(1),Some(2),Some(2),Some(3),Some(3),None,None,Some(4),Some(4)];
        let root = TreeNode::build(root);
        println!("root ====={:#?}",root);
        assert_eq!(CalculateDepth::calculate_depth(root),4);

    }
}
