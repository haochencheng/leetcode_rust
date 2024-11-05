use crate::tree::list_node_box::ListNode;

// LCR 142. 训练计划 IV
pub trait Solution {
    fn trainning_plan(head: Option<Box<ListNode>>, cnt: i32) -> Option<Box<ListNode>>;
}

pub struct TrainningPlan {}

impl Solution for TrainningPlan {
    fn trainning_plan(head: Option<Box<ListNode>>, cnt: i32) -> Option<Box<ListNode>> {
        if head.is_none() {
            return None;
        }
        let mut fast = 0;
        let mut current = &head;
        let mut slow_node = &head;
        while let Some(ref node) = current {
            if fast >= cnt {
                slow_node = &slow_node.as_ref().unwrap().next;
            }
            current = &node.next;
            fast += 1;
        }
        slow_node.clone()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn judge_circle() {
        let arr: Vec<i32> = vec![5, 4, 3, 2, 1];
        let cnt: i32 = 3;
        let arr_node = ListNode::build(arr.clone());
        let result = ListNode::get_by_index(&arr_node, arr.len() as i32 - cnt);
        let ans = TrainningPlan::trainning_plan(arr_node, cnt);
        if let Some(node) = ans.clone() {
            println!("ans===={}", node);
        }
        if let Some(node) = result.clone() {
            println!("result===={}", node);
        }
        assert_eq!(ans, result);
    }
}
