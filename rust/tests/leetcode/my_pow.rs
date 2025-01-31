use assert_approx_eq::assert_approx_eq;

#[test]
fn test_my_pow() {
    use practice::leetcode::my_pow::Solution;
    let cases = vec![
        (2.0, 10, 1024.0),
        (2.1, 3, 9.261),
        (2.0, -2, 0.25),
        (2.0, 0, 1.0),
        (2.0, 1, 2.0),
    ];
    for (x, n, expected) in cases {
        let result = Solution::my_pow(x, n);
        assert_approx_eq!(result, expected);
    }
}
