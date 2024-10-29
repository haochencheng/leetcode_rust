// LCR 176. 判断是否为平衡二叉树
use std::rc::Rc;
use std::cell::RefCell;
use crate::tree::tree_node::TreeNode;

pub trait Solution {
    fn find_target_node(root: Option<Rc<RefCell<TreeNode>>>,cnt: i32) -> i32;

}


pub struct FindTargetNode {}

impl Solution for FindTargetNode {

    fn find_target_node(root: Option<Rc<RefCell<TreeNode>>> ,cnt: i32) -> i32 {
        if root.is_none(){
            return 0;
        }
        // fn scan_tree_node(node: &Option<Rc<RefCell<TreeNode>>> ,cnt: i32,arr:&mut Vec<i32>)  -> (){
        //     if let Some(node) = node {
        //         let val=&node.borrow().val;
        //         arr.push(*val);
        //         if arr.len()==cnt as usize {
        //             return ;
        //         }else {
        //             scan_tree_node(&node.borrow().right, cnt, arr);
        //             scan_tree_node(&node.borrow().left, cnt, arr);
        //         }
        //     }
        // }
        // let mut arr=Vec::new();
        // scan_tree_node(&root, cnt, &mut arr);
        // arr.sort();
        // arr[cnt as usize-1]
    }

   
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::tree::tree_node::TreeNode;

    #[test]
    fn judge_circle() {
        let root  = vec![Some(7),Some(3),Some(9),Some(1),Some(5)];
        let root = TreeNode::build(root);
        // println!("root ====={:#?}",root);
        assert_eq!(FindTargetNode::find_target_node(root,2),7);

    }
}
