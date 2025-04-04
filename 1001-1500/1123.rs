// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
// 
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    fn solve(root: Option<Rc<RefCell<TreeNode>>>) -> (i32, Option<Rc<RefCell<TreeNode>>>) {
        if root.is_none() {
            return (0, None);
        }
        if let Some(r) = root.clone() {
            let rb = r.borrow();
            let (h1, l1) = Self::solve(rb.left.clone());
            let (h2, l2) = Self::solve(rb.right.clone());
            if h1 > h2 {
                return (h1+1, l1);
            }
            if h2 > h1 {
                return (h2+1, l2);
            }
            return (h1+1, root);
        }
        unreachable!();
    }
    pub fn lca_deepest_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::solve(root).1
    }
}
