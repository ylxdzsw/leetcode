// You are given two non-empty linked lists representing two non-negative integers. The digits are stored in reverse order and each of their nodes contain a single digit. Add the two numbers and return it as a linked list.

// You may assume the two numbers do not contain any leading zero, except the number 0 itself.

// Example:

// Input: (2 -> 4 -> 3) + (5 -> 6 -> 4)
// Output: 7 -> 0 -> 8
// Explanation: 342 + 465 = 807.

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

fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut root = ListNode::new(0);
    let mut residual = 0;
    let mut curr = &mut root;

    let mut l = l1;
    let mut r = l2;

    loop {
        let mut x = match (&l, &r) {
            (Some(a), Some(b)) => a.val + b.val + residual,
            (Some(a), None) | (None, Some(a)) => a.val + residual,
            _ => {
                if residual != 0 {
                    curr.next = Some(Box::new(ListNode::new(residual)))
                }
                return root.next
            }
        };

        if x >= 10 {
            residual = x / 10;
            x %= 10
        } else {
            residual = 0
        }

        curr.next = Some(Box::new(ListNode::new(x)));
        curr = curr.next.as_mut().unwrap();

        l = l.and_then(|x| x.next);
        r = r.and_then(|x| x.next)
    }
}

fn main() {
    let a = Some(Box::new(ListNode {
        val: 2, next: Some(Box::new(ListNode {
            val: 4, next: Some(Box::new(ListNode {
                val: 3, next: None
            }))
        }))
    }));

    let b = Some(Box::new(ListNode {
        val: 5, next: Some(Box::new(ListNode {
            val: 6, next: Some(Box::new(ListNode {
                val: 4, next: None
            }))
        }))
    }));

    let r = Some(Box::new(ListNode {
        val: 7, next: Some(Box::new(ListNode {
            val: 0, next: Some(Box::new(ListNode {
                val: 8, next: None
            }))
        }))
    }));

    assert_eq!(add_two_numbers(a, b), r)
}

// faster than 85.93%, less memory than 100%.