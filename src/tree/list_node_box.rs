use std::borrow::BorrowMut;
use std::fmt;
// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }

    pub fn build(values: Vec<i32>) -> Option<Box<ListNode>> {
        if values.is_empty() {
            return None;
        }
        // 从后向前构建链表，初始时链表为空
        let mut current = None;

        // 迭代每个值，从末尾向前
        for &value in values.iter().rev() {
            // 创建新节点，其 next 指向当前链表的头节点
            let new_node = Box::new(ListNode {
                val: value,
                next: current,
            });
            // 更新当前链表头为新节点
            current = Some(new_node);
        }
        current
    }

    // display 方法将返回当前节点的字符串表示
    pub fn display(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} => ", self.val)?;

        // 确保 node 的类型被清晰推导
        if let Some(node) = &self.next {
            // 这里直接调用 borrow() 的结果，确保编译器能推导类型
            node.display(f)
        } else {
            write!(f, "None") // 如果没有下一个节点，输出 "None"
        }
    }
}

// 实现 Display trait 以支持格式化输出
impl fmt::Display for ListNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.display(f)
    }
}
