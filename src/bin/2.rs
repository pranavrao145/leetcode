// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
//
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut l1_pointer = &l1;
        let mut l2_pointer = &l2;
        let mut carry = 0;

        let mut res = None;
        let mut curr_node = &mut res;

        while l1_pointer.is_some() || l2_pointer.is_some() || carry != 0 {
            let mut digit_sum = carry;

            if let Some(node) = l1_pointer {
                digit_sum += node.val;
                l1_pointer = &node.next;
            }

            if let Some(node) = l2_pointer {
                digit_sum += node.val;
                l2_pointer = &node.next;
            }

            carry = digit_sum / 10;
            *curr_node = Some(Box::new(ListNode::new(digit_sum % 10)));
            curr_node = &mut curr_node.as_mut().unwrap().next;
        }

        res
    }
}
