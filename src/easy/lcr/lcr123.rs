use crate::tree::list_node_box::ListNode;

// //
pub trait Solution {
    fn reverse_book_list(head: Option<Box<ListNode>>) -> Vec<i32>;
}

pub struct ReverseBookList {}

impl Solution for ReverseBookList {
    fn reverse_book_list(head: Option<Box<ListNode>>) -> Vec<i32> {
        if head.is_none() {
            return Vec::new();
        }
        let mut current = head;
        let mut ans = Vec::new();
        while let Some(node) = current {
            ans.push(node.val);
            current = node.next
        }
        ans.reverse();
        ans
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn judge_circle() {
        let head = vec![3, 6, 4, 1];
        let result = vec![1, 4, 6, 3];
        assert_eq!(
            ReverseBookList::reverse_book_list(ListNode::build(head)),
            result
        );
    }
}
