use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

type NodeRef = Rc<RefCell<TreeNode>>;

impl Solution {
    pub fn is_even_odd_tree(root: Option<NodeRef>) -> bool {
        if root.is_none() {
            return false;
        }

        let mut level = 0;
        let (mut queue, mut temp) = (VecDeque::new(), VecDeque::new());
        queue.push_back(root.unwrap());

        while !queue.is_empty() {
            let mut prev = if level % 2 == 0 { i32::MIN } else { i32::MAX };
            while let Some(n) = queue.pop_front() {
                let node = n.borrow();
                if (level % 2 == 0 && (node.val % 2 == 0 || node.val <= prev))
                    || (level % 2 == 1 && (node.val % 2 == 1 || node.val >= prev))
                {
                    return false;
                }
                prev = node.val;
                if let Some(l) = node.left.clone() {
                    temp.push_back(l)
                }
                if let Some(r) = node.right.clone() {
                    temp.push_back(r)
                }
            }
            std::mem::swap(&mut temp, &mut queue);
            level += 1;
        }

        true
    }
}
