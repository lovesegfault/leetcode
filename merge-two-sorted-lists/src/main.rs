#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

struct Solution;

impl Solution {
    pub fn merge_two_lists(
        mut l1: Option<Box<ListNode>>,
        mut l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        match (&l1, &l2) {
            (None, None) => None,
            (Some(l), None) | (None, Some(l)) => Some(l.clone()),
            _ => {
                let node1 = l1.as_mut().unwrap();
                let node2 = l2.as_mut().unwrap();

                if node1.val < node2.val {
                    node1.next = Solution::merge_two_lists(node1.next.take(), l2);
                    l1
                } else {
                    node2.next = Solution::merge_two_lists(l1, node2.next.take());
                    l2
                }
            }
        }
    }

    pub fn merge_two_lists2(
        mut l1: Option<Box<ListNode>>,
        mut l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut small = &mut l1;
        let big = &mut l2;
        while big.is_some() {
            if small.is_none() || small.as_ref()?.val > big.as_ref()?.val {
                std::mem::swap(small, big);
            }
            if small.is_some() {
                small = &mut small.as_mut()?.next;
            }
        }
        l1
    }
}

fn main() {
    println!("Hello, world!");
}
