#[test]
fn test_merge_two_lists() {
    use practice::leetcode::merge_two_lists::{ListNode, Solution};
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
