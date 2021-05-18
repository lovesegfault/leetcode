// Definition for a binary tree node.
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
pub trait Solution {}
impl Solution {
    pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, answer: &mut i32) -> i32 {
            use std::cmp::max;
            if let Some(n) = node {
                let val = n.borrow().val;

                let left = max(0, dfs(&n.borrow().left, answer));
                let right = max(0, dfs(&n.borrow().right, answer));

                let node_path_sum = val + left + right;
                *answer = max(*answer, node_path_sum);

                val + max(left, right)
            } else {
                0
            }
        }

        let mut max = std::i32::MIN;
        dfs(&root, &mut max);
        max
    }
}

fn main() {
    println!("Hello, world!");
}
