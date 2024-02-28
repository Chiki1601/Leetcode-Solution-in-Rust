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
    pub fn find_bottom_left_value(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn find(root: Option<Rc<RefCell<TreeNode>>>, height: i32, max: &mut i32, res: &mut i32) {
            match root {
                Some(root) => {
                    if height > *max {
                        *max = height;
                        *res = root.borrow().val;
                    }
                    find(root.borrow().left.clone(), height+1, max, res);
                    find(root.borrow().right.clone(), height+1, max, res);
                }
                _ => (),
            }
        }
        let (max, res) = (&mut 0, &mut 0);
        find(root.clone(), 1, max, res);
        *res
    }
}
