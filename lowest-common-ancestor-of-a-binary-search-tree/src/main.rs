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

use std::borrow::Borrow;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    #[inline(always)]
    fn node_val(n: Option<&Rc<RefCell<TreeNode>>>) -> Option<i32> {
        Some(n?.as_ref().borrow().val)
    }

    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let r_val = Self::node_val(root.as_ref())?;
        let p_val = Self::node_val(p.as_ref())?;
        let q_val = Self::node_val(q.as_ref())?;

        use std::cmp::Ordering::*;
        match (p_val.cmp(&r_val), q_val.cmp(&r_val)) {
            (Greater, Greater) => {
                let right = root
                    .as_ref()
                    .and_then(|r| r.as_ref().borrow().right.clone());
                Self::lowest_common_ancestor(right, p, q)
            }
            (Less, Less) => {
                let left = root.as_ref().and_then(|r| r.as_ref().borrow().left.clone());
                Self::lowest_common_ancestor(left, p, q)
            }
            _ => root,
        }
    }
}

fn main() {
    println!("Hello, world!");
}
