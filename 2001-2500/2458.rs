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
use std::collections::HashMap;
impl Solution {
    fn map(node: Option<Rc<RefCell<TreeNode>>>, levels: &mut Vec<Vec<(i32, usize)>>, map: &mut HashMap<i32, usize>, level: usize) -> usize {
        if let Some(node_rc) = node {
            let node_ref = node_rc.borrow();
            if levels.len() == level {
                levels.push(vec![]);
            }
            let max_level = Self::map(node_ref.left.clone(), levels, map, level + 1)
                .max(Self::map(node_ref.right.clone(), levels, map, level + 1));
            levels[level].push((node_ref.val, max_level));
            map.insert(node_ref.val, level);
            max_level
        } else {
            level - 1
        }
    }

    pub fn tree_queries(root: Option<Rc<RefCell<TreeNode>>>, queries: Vec<i32>) -> Vec<i32> {
        let mut levels = vec![];
        let mut map = HashMap::new();
        Self::map(root, &mut levels, &mut map, 0);
        queries.into_iter().map(|q| {
            let &level = map.get(&q).unwrap();
            if levels[level].len() == 1 {
                (level - 1) as i32
            } else {
                levels[level].iter().filter_map(|(val, l)| (*val != q).then(|| {
                    *l
                })).max().unwrap() as i32
            }
        }).collect()
    }
}
