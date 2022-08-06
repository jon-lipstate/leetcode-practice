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

pub fn merge_two_lists(
    list1: Option<Box<ListNode>>,
    list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    unimplemented!();
    let mut head: Option<Box<ListNode>> = None;
    let mut tail: &Option<Box<ListNode>> = &None;
    let mut left: Option<Box<ListNode>> = list1;
    let mut right: Option<Box<ListNode>> = list2;
    loop {
        let v_left = match &left {
            None => None,
            Some(l) => Some(l.val),
        };
        let v_right = match &right {
            None => None,
            Some(r) => Some(r.val),
        };
        match left_is_less(v_left, v_right) {
            Err(_) => {
                return head;
            }
            Ok(b) => match tail {
                None => match b {
                    true => {
                        left = left.unwrap().next;
                        tail = Some(Box::new(ListNode::new(v_left.unwrap())));
                    }
                    false => {
                        right = right.unwrap().next;
                        tail = Some(Box::new(ListNode::new(v_right.unwrap())));
                    }
                },
                Some(_) => match b {
                    true => {
                        left = left.unwrap().next;
                        tail.unwrap().next = Some(Box::new(ListNode::new(v_left.unwrap())));
                    }
                    false => {
                        right = right.unwrap().next;
                        tail.unwrap().next = Some(Box::new(ListNode::new(v_right.unwrap())));
                    }
                },
            },
        }
        if head.is_none() && tail.is_some() {
            head = tail.clone();
        }
    }
    return head;
}

fn left_is_less(left: Option<i32>, right: Option<i32>) -> Result<bool, ()> {
    if left.is_some() && right.is_some() {
        if left.is_some() && right.is_some() {
            if left.unwrap() > right.unwrap() {
                return Ok(false);
            } else {
                return Ok(true);
            }
        } else if left.is_some() {
            return Ok(true);
        } else if right.is_some() {
            return Ok(false);
        }
    }
    Err(())
}
