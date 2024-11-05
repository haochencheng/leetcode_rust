use crate::tree::list_node_box::ListNode;
//
pub trait Solution {
    fn delete_node(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>>;
}

pub struct TrainningPlan {}

impl Solution for TrainningPlan {
    fn delete_node(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        if head.is_none() {
            return None;
        }
        let mut head = Some(Box::new(ListNode {
            val: -1,
            next: head.clone(),
        }));
        let mut current = &mut head;
        while let Some(ref mut node) = current {
            let next = &mut node.next;
            if let Some(next_node) = next {
                if next_node.val == val {
                    node.next = next_node.next.take();
                }
            }

            current = &mut node.next;
        }
        head.unwrap().next
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn judge_circle() {
        let arr: Vec<i32> = vec![4, 5, 1, 9];
        let head = ListNode::build(arr.clone());
        let result = ListNode::build(vec![4, 1, 9]);
        let ans = TrainningPlan::delete_node(head, 5);
        if let Some(ans) = ans.clone() {
            println!("ans============{}", ans);
        }
        assert_eq!(ans, result);
    }
}
