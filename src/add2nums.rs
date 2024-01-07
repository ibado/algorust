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

pub fn add_two_numbers(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    match (l1, l2) {
        (None, None) => None,
        (Some(ln), None) => Some(ln),
        (None, Some(ln)) => Some(ln),
        (Some(ln1), Some(ln2)) => {
            let sum = ln1.val + ln2.val;
            if sum < 10 {
                let mut ln = ListNode::new(sum);
                ln.next = add_two_numbers(ln1.next, ln2.next);
                Some(Box::new(ln))
            } else {
                let last = Some(Box::new(ListNode::new(1)));
                let mut blast = Box::new(ListNode::new(sum % 10));
                blast.next = add_two_numbers(add_two_numbers(last, ln1.next), ln2.next);
                Some(blast)
            }
        }
    }
}

pub fn run(execute: bool) {
    if !execute {
        return;
    }
    let mut l1_0 = Box::new(ListNode::new(2));
    let mut l1_1 = Box::new(ListNode::new(1));
    let mut l1_2 = Box::new(ListNode::new(9));
    let l1_3 = Box::new(ListNode::new(1));
    l1_2.next = Some(l1_3);
    l1_1.next = Some(l1_2);
    l1_0.next = Some(l1_1);

    let mut l2_0 = Box::new(ListNode::new(1));
    let mut l2_1 = Box::new(ListNode::new(2));
    let l2_2 = Box::new(ListNode::new(3));
    l2_1.next = Some(l2_2);
    l2_0.next = Some(l2_1);

    let r = add_two_numbers(Some(l1_0), Some(l2_0));
    println!("result: {:?}", r);
}
