struct Solution;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn max_depth(root: Option<Box<TreeNode>>) -> i32 {
        let mut max_depth = 0;
        let mut current_depth = 0;

        fn traversal(node: &Option<Box<TreeNode>>, current_depth: &mut i32, max_depth: &mut i32) {
            if let Some(node_ref) = node {
                *current_depth += 1;

                if node_ref.left.is_none() && node_ref.right.is_none() {
                    *max_depth = (*current_depth).max(*max_depth);
                }

                traversal(&node_ref.left, current_depth, max_depth);
                traversal(&node_ref.right, current_depth, max_depth);

                *current_depth -= 1;
            }
        }

        traversal(&root, &mut current_depth, &mut max_depth);
        max_depth
    }
}
