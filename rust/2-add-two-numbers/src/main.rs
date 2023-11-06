struct Solution;

struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut carry = 0;
        let mut p1 = &l1;
        let mut p2 = &l2;
        let mut result = None;
        let mut p3 = &mut result;

        while p1.is_some() || p2.is_some() || carry > 0 {
            let mut sum = carry;
            if let Some(node) = p1 {
                sum += node.val;
                p1 = &node.next;
            }
            if let Some(node) = p2 {
                sum += node.val;
                p2 = &node.next;
            }
            carry = sum / 10;
            *p3 = Some(Box::new(ListNode::new(sum % 10)));
            p3 = &mut p3.as_mut().unwrap().next;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            Solution::add_two_numbers(
                Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode {
                        val: 4,
                        next: Some(Box::new(ListNode::new(3))),
                    })),
                })),
                Some(Box::new(ListNode {
                    val: 5,
                    next: Some(Box::new(ListNode {
                        val: 6,
                        next: Some(Box::new(ListNode::new(4))),
                    })),
                })),
            ),
            Some(Box::new(ListNode {
                val: 7,
                next: Some(Box::new(ListNode {
                    val: 0,
                    next: Some(Box::new(ListNode::new(8))),
                })),
            })),
        );
    }
}
