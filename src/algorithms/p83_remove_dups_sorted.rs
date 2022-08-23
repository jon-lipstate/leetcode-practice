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
// pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
//     use std::collections::HashSet;
//     if head.is_none() {
//         return None;
//     }
//     let mut h = HashSet::new(); // use for uniqueness
//     let mut ordered = Vec::new(); //use for ordering
//     let mut current = head;
//     while current.is_some() {
//         let uc = current.unwrap();
//         let v = uc.val;
//         match h.insert(v) {
//             true => ordered.push(v),
//             false => {}
//         };
//         current = uc.next;
//     }
//     let mut new_head: Option<Box<ListNode>> = Some(Box::new(ListNode {
//         val: ordered[0],
//         next: None,
//     }));
//     let mut prev_node = &mut new_head;
//     ordered.remove(0);

//     for i in ordered {
//         let mut p = prev_node.as_mut().unwrap();
//         p.next = Some(Box::new(ListNode { val: i, next: None }));
//         prev_node = &mut p.next;
//     }

//     new_head
// }

//0ms soln
pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    if head.is_none() {
        return None;
    }

    let mut head = head;
    let mut current = head.as_mut().unwrap();

    while let Some(next) = current.next.as_mut() {
        if current.val == next.val {
            current.next = next.next.take();
        } else {
            current = current.next.as_mut().unwrap();
        }
    }

    head
}
