use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Rc<RefCell<ListNode>>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }

    pub fn build(values: Vec<i32>) -> Option<Rc<RefCell<ListNode>>> {
        if values.is_empty() {
            return None;
        }
        let root = Rc::new(RefCell::new(ListNode::new(values[0])));
        let mut queue = vec![Rc::clone(&root)];
        for i in 1..values.len() {
            let next = Rc::new(RefCell::new(ListNode::new(values[i])));
            let cur: Rc<RefCell<ListNode>> = queue.remove(0);
            cur.borrow_mut().next = Some(Rc::clone(&next));
            queue.push(next);
        }
        Some(root)
    }

    pub fn build_with_list_node(
        values: Vec<i32>,
        list_node: &Option<Rc<RefCell<ListNode>>>,
    ) -> Option<Rc<RefCell<ListNode>>> {
        if values.is_empty() {
            return None;
        }
        let root = Rc::new(RefCell::new(ListNode::new(values[0])));
        let mut queue = vec![Rc::clone(&root)];
        for i in 1..values.len() {
            let next = Rc::new(RefCell::new(ListNode::new(values[i])));
            let cur: Rc<RefCell<ListNode>> = queue.remove(0);
            cur.borrow_mut().next = Some(Rc::clone(&next));
            queue.push(next);
        }
        let cur: Rc<RefCell<ListNode>> = queue.remove(0);
        if let Some(node) = list_node {
            cur.borrow_mut().next = Some(Rc::clone(&node));
        }
        Some(root)
    }

    // pub fn get_last_node(head: Option<Rc<RefCell<ListNode>>>) -> Option<Rc<RefCell<ListNode>>> {
    //     let mut current = head;

    //     while let Some(node) = current {
    //         // 获取当前节点的下一个节点
    //         current = node.as_ref().borrow().next.clone();
    //     }

    //     // 当前节点为 None，返回最后一个非 None 的节点
    //     if let Some(node) = current {
    //         Some(node)
    //     } else {
    //         None
    //     }
    // }

    // display 方法将返回当前节点的字符串表示
    pub fn display(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} => ", self.val)?;

        // 确保 node 的类型被清晰推导
        if let Some(node) = &self.next {
            // 这里直接调用 borrow() 的结果，确保编译器能推导类型
            node.as_ref().borrow().display(f)
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
