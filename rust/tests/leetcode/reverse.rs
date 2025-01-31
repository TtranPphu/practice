#[test]
fn test_reverse() {
    use practice::leetcode::reverse::Solution;
    assert_eq!(Solution::reverse(123), 321);
    assert_eq!(Solution::reverse(-123), -321);
    assert_eq!(Solution::reverse(120), 21);
    assert_eq!(Solution::reverse(0), 0);
    assert_eq!(Solution::reverse(1534236469), 0);
    assert_eq!(Solution::reverse(-2147483648), 0);
}
