use crate::tree::list_node_box::ListNode;

// LCR 142. 训练计划 IV
pub trait Solution {
    fn trainning_plan(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>>;
}

pub struct TrainningPlan {}

impl Solution for TrainningPlan {
    fn trainning_plan(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        match (l1, l2) {
            (Some(node1), None) => Some(node1),
            (None, Some(node2)) => Some(node2),
            (Some(mut node1), Some(mut node2)) => {
                if node1.val < node2.val {
                    let n = node1.next.take();
                    node1.next = Self::trainning_plan(n, Some(node2));
                    Some(node1)
                } else {
                    let n = node2.next.take();
                    node2.next = Self::trainning_plan(Some(node1), n);
                    Some(node2)
                }
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn judge_circle() {
        let (l1, l2) = (
            ListNode::build(vec![1, 2, 4]),
            ListNode::build(vec![1, 3, 4]),
        );
        let result = ListNode::build(vec![1, 1, 2, 3, 4, 4]);
        match result {
            Some(ref list) => println!("======result: {}", list), // 解包并打印链表
            None => println!("======result: Empty List"),
        }
        assert_eq!(TrainningPlan::trainning_plan(l1, l2), result);
    }
}
