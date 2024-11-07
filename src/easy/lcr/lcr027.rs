use std::collections::VecDeque;

use crate::tree::list_node_box::ListNode;

struct Solution {}

impl Solution {
    pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
        if head.is_none() {
            return false;
        }
        let mut current = head;
        let mut deque = VecDeque::new();
        while let Some(node) = current {
            deque.push_back(node.val);
            current = node.next;
        }
        if deque.len() == 1 {
            return true;
        }
        while deque.len() > 1 {
            if deque.pop_back().unwrap() != deque.pop_front().unwrap() {
                return false;
            }
        }
        return true;
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn is_palindrome() {
        let head = ListNode::build(vec![1, 2, 3, 3, 2, 1]);
        assert!(Solution::is_palindrome(head));
    }
}
