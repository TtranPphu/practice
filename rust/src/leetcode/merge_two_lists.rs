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
    #[allow(dead_code)]
    pub fn from_vec(vals: Vec<i32>) -> Option<Box<Self>> {
        let mut node: Option<Box<ListNode>> = None;
        for val in vals.into_iter().rev() {
            let mut new_node = ListNode::new(val);
            new_node.next = node;
            node = Some(Box::new(new_node));
        }
        node
    }
    #[allow(dead_code)]
    pub fn to_vec(&self) -> Vec<i32> {
        let mut node = self;
        let mut vals = Vec::new();
        loop {
            vals.push(node.val);
            node = match &node.next {
                Some(n) => n,
                None => break,
            };
        }
        vals
    }
}
pub struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        return Self::merge_two_lists_v2(list1, list2);
    }
    #[allow(dead_code)]
    pub fn merge_two_lists_v2(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut head: Option<Box<ListNode>> = None;
        let mut l1 = list1;
        let mut l2 = list2;
        let mut tail = &mut head;
        loop {
            let val;
            match (l1.clone(), l2.clone()) {
                (Some(n1), Some(n2)) => {
                    if n1.val < n2.val {
                        val = n1.val;
                        l1 = n1.next;
                    } else {
                        val = n2.val;
                        l2 = n2.next;
                    }
                }
                (Some(n1), None) => {
                    val = n1.val;
                    l1 = n1.next;
                }
                (None, Some(n2)) => {
                    val = n2.val;
                    l2 = n2.next;
                }
                (None, None) => break,
            }

            let node = Some(Box::new(ListNode::new(val)));

            match tail {
                Some(t) => {
                    t.next = node;
                    tail = &mut t.next;
                }
                None => {
                    head = node;
                    tail = &mut head;
                }
            }
        }
        head
    }
    #[allow(dead_code)]
    pub fn merge_two_lists_v1(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        match (list1, list2) {
            (None, None) => None,
            (Some(l), None) | (None, Some(l)) => Some(l),
            (Some(l1), Some(l2)) => {
                if l1.val > l2.val {
                    Some(Box::new(ListNode {
                        val: l2.val,
                        next: Self::merge_two_lists(Some(l1), l2.next),
                    }))
                } else {
                    Some(Box::new(ListNode {
                        val: l1.val,
                        next: Self::merge_two_lists(l1.next, Some(l2)),
                    }))
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ListNode;
    use super::Solution;

    #[test]
    fn test_merge_two_lists() {
        let list1 = ListNode::from_vec(vec![1, 2, 4]);
        let list2 = ListNode::from_vec(vec![1, 3, 4]);
        let r = Solution::merge_two_lists(list1, list2);
        let rr = if r.is_some() {
            r.unwrap().to_vec()
        } else {
            vec![]
        };
        let expected = vec![1, 1, 2, 3, 4, 4];
        assert_eq!(rr, expected);
    }
}
