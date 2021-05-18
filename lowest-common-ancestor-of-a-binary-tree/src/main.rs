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
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        #[inline]
        fn inner(root: Rc<RefCell<TreeNode>>, vp: i32, vq: i32) -> Option<Rc<RefCell<TreeNode>>> {
            let vroot = root.borrow().val;
            if vroot == vp || vroot == vq {
                return Some(root);
            }

            let left = root
                .borrow()
                .left
                .as_ref()
                .and_then(|l| inner(l.clone(), vp, vq));

            let right = root
                .borrow()
                .right
                .as_ref()
                .and_then(|r| inner(r.clone(), vp, vq));

            match (left, right) {
                (Some(_), Some(_)) => Some(root),
                (None, None) => None,
                (Some(n), None) | (None, Some(n)) => Some(n),
            }
        }

        let root = root?;
        let vp = p?.as_ref().borrow().val;
        let vq = q?.as_ref().borrow().val;
        inner(root, vp, vq)
    }
}

fn main() {
    println!("Hello, world!");
}
