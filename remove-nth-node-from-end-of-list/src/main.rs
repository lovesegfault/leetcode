struct Solution;

// Definition for singly-linked list.
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

// n = 4
// 0 -> 1 -> 2 -> 3 -> 4 -> 5 -> 6 -> 7 -> 8 -> 9
//

impl Solution {
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut dummy = ListNode::new(0);
        dummy.next = head;
        let mut dummy = Box::new(dummy);
        let mut fast = dummy.clone();
        let mut slow = dummy.as_mut();
        for _ in 0..n {
            fast = fast.next.unwrap();
        }

        while fast.next.is_some() {
            fast = fast.next.unwrap();
            slow = slow.next.as_mut().unwrap();
        }
        let next = slow.next.as_mut().unwrap();
        slow.next = next.next.clone();
        dummy.next
    }
}

fn main() {
    println!("Hello, world!");
}
