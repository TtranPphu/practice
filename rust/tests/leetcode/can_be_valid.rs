#[test]
fn test_can_be_valid() {
    use practice::leetcode::can_be_valid::Solution;
    assert!(Solution::can_be_valid("))()))".to_string(), "010100".to_string()) == true);
    assert!(Solution::can_be_valid("()()".to_string(), "0000".to_string()) == true);
    assert!(Solution::can_be_valid(")".to_string(), "0".to_string()) == false);
    assert!(
        Solution::can_be_valid(
            "())()))()(()(((())(()()))))((((()())(())".to_string(),
            "1011101100010001001011000000110010100101".to_string()
        ) == true
    );
    assert!(
        Solution::can_be_valid(
            "())(()(()(())()())(())((())(()())((())))))(((((((())(()))))(".to_string(),
            "100011110110011011010111100111011101111110000101001101001111".to_string()
        ) == false
    );
}
