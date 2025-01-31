#[test]
fn test_solve_n_queens() {
    use practice::leetcode::solve_n_queens::Solution;
    let cases = vec![
        (
            4,
            vec![
                vec![".Q..", "...Q", "Q...", "..Q."],
                vec!["..Q.", "Q...", "...Q", ".Q.."],
            ],
        ),
        (2, vec![]),
    ];
    for (n, expected) in cases {
        let result = Solution::solve_n_queens(n);
        assert_eq!(result, expected);
    }
}
