#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i64,
    pub next: Option<Box<ListNode>>,
}

#[allow(dead_code)]
pub fn merge(
    mut list1: Option<Box<ListNode>>,
    mut list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut head = None;
    let mut next_tail = &mut head;

    while list1.is_some() && list2.is_some() {
        let head1 = &mut list1;
        let head2 = &mut list2;

        let input_head = if head1.as_ref().unwrap().val < head2.as_ref().unwrap().val {
            head1
        } else {
            head2
        };

        std::mem::swap(input_head, next_tail);

        let next_tail_next = &mut next_tail.as_mut().unwrap().next;
        std::mem::swap(input_head, next_tail_next);
        next_tail = next_tail_next;
    }

    *next_tail = if list1.is_some() { list1 } else { list2 };
    head
}

#[test]
pub fn merge_test() {
    let head_a = Some(Box::new(ListNode { val: 2, next: None }));
    let head_b = Some(Box::new(ListNode { val: 1, next: None }));

    let mut result = merge(head_a, head_b);

    for i in 1..3 {
        match result.clone() {
            None => panic!("broken link!"),
            Some(node) => {
                assert_eq!(node.val, i);
            }
        }
        result = result.clone().unwrap().next;
    }
}
