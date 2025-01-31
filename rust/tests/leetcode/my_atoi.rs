#[test]
fn test_my_atoi() {
    use practice::leetcode::my_atoi::Solution;
    let cases = vec![
        ("42", 42),
        ("   -42", -42),
        ("4193 with words", 4193),
        ("words and 987", 0),
        ("-91283472332", -2147483648),
        ("123-456", 123),
        ("-21474836482", -2147483648),
    ];
    for (s, expected) in cases {
        let result = Solution::my_atoi(s.to_string());
        assert_eq!(result, expected);
    }
}
