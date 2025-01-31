#[test]
fn test_monkey_move() {
    use practice::leetcode::monkey_move::Solution;
    assert_eq!(Solution::monkey_move(3), 6);
    assert_eq!(Solution::monkey_move(4), 14);
    assert_eq!(Solution::monkey_move(55), 766762394);
    assert_eq!(Solution::monkey_move(500000004), 0);
}
