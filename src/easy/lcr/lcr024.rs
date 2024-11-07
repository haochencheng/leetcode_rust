use crate::tree::list_node_box::ListNode;

struct Solution {}

impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() {
            return None;
        }
        // 哨兵节点
        let mut prev = None; // 用于存储翻转后的链表
        let mut current = head; // 用于遍历原链表

        while let Some(mut node) = current {
            // 把当前节点的 next 指针指向 prev，从而实现翻转
            let next = node.next.take(); // 断开当前节点的 next 引用
            node.next = prev;

            // 更新 prev 和 current 指针
            prev = Some(node);
            current = next;
        }

        prev
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn is_palindrome() {
        let head = ListNode::build(vec![1, 2, 3, 4, 5]);
        let ans = ListNode::build(vec![5, 4, 3, 2, 1]);
        assert_eq!(Solution::reverse_list(head), ans);
    }
}
