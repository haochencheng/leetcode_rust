use crate::tree::list_node_box::ListNode;

// LCR 142. 训练计划 IV
pub trait Solution {
    fn trainning_plan(head: Option<Box<ListNode>>) -> Option<Box<ListNode>>;
}

pub struct TrainningPlan {}

impl Solution for TrainningPlan {
    fn trainning_plan(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        // 初始化前一个节点为 None，这是翻转后的链表的尾部
        let mut prev = None;
        let mut current = head;

        while let Some(mut node) = current {
            // 暂存当前节点的下一个节点
            let next = node.next.take();
            // 反转当前节点的指向，使其指向前一个节点
            node.next = prev;
            // 更新 prev 和 current，以便继续遍历
            prev = Some(node);
            current = next;
        }

        // 返回 prev，它是反转后的新链表的头节点
        prev
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn judge_circle() {
        let result = ListNode::build(vec![5, 4, 3, 2, 1]);
        let ans = TrainningPlan::trainning_plan(ListNode::build(vec![1, 2, 3, 4, 5]));
        if let Some(node) = ans.clone() {
            println!("ans===={}", node);
        }
        assert_eq!(ans, result);
    }
}
