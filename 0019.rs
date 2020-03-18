// Given a linked list, remove the n-th node from the end of list and return its head.

// Example:

// Given linked list: 1->2->3->4->5, and n = 2.

// After removing the second node from the end, the linked list becomes 1->2->3->5.
// Note:

// Given n will always be valid.

// Follow up:

// Could you do this in one pass?

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
    fn _remove_nth_from_end(node: &mut ListNode, n: i32) -> i32 {
        if node.next.is_none() {
            return 1
        }

        let l = _remove_nth_from_end(node.next.as_mut().unwrap(), n);
        if l == n {
            let mut next = node.next.take().unwrap();
            node.next = next.next.take();
        }

        l + 1
    }

    let mut root = ListNode { val: 0, next: head };
    _remove_nth_from_end(&mut root, n);
    root.next
}

fn make_list(x: &[i32]) -> Option<Box<ListNode>> {
    if x.is_empty() {
        return None
    }

    Some(Box::new(ListNode { val: x[0], next: make_list(&x[1..]) }))
}

fn main() {
    let a = make_list(&[1, 2, 3, 4, 5]);
    let b = make_list(&[1, 2, 3, 5]);
    assert_eq!(remove_nth_from_end(a, 2), b);
}

// faster than 100.00%, less memory than 100.00%.
// could use a fixed-sized stack, but we had to fight the borrow checker. Anyway it already takes only 0ms XD.