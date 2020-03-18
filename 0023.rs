// Merge k sorted linked lists and return it as one sorted list. Analyze and describe its complexity.

// Example:

// Input:
// [
//   1->4->5,
//   1->3->4,
//   2->6
// ]
// Output: 1->1->2->3->4->4->5->6

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

fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
    let mut lists = lists;
    let mut heap_buffer = std::collections::BinaryHeap::with_capacity(lists.len());
    let mut root = ListNode::new(0);
    let mut last = &mut root;

    // heap initialization
    for (i, l) in lists.iter_mut().enumerate() {
        if let Some(mut node) = l.take() { // TODO: remove Nones from lists?
            *l = node.next.take();
            heap_buffer.push((-node.val, i)) // Rust's heap is the rediculars max heap
        }
    }

    while let Some((v, i)) = heap_buffer.pop() {
        last.next = Some(Box::new(ListNode::new(-v)));
        last = last.next.as_mut().unwrap();
        if let Some(mut node) = lists[i].take() {
            lists[i] = node.next.take();
            heap_buffer.push((-node.val, i))
        }
    }

    root.next
}

fn make_list(x: &[i32]) -> Option<Box<ListNode>> {
    if x.is_empty() {
        return None
    }

    Some(Box::new(ListNode { val: x[0], next: make_list(&x[1..]) }))
}

fn main() {
    let a = make_list(&[1, 4, 5]);
    let b = make_list(&[1, 3, 4]);
    let c = make_list(&[2, 6]);
    let ans = make_list(&[1, 1, 2, 3, 4, 4, 5, 6]);
    assert_eq!(merge_k_lists(vec![a, b, c]), ans);
}

// faster than 100.00%, less memory than 100.00%.