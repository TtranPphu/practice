// #[test]
fn test_count_non_decreasing_subarrays() {
    use practice::leetcode::cnds::Solution;
    let cases = vec![
        (vec![6, 3, 1, 2, 4, 4], 7, 17),
        (vec![6, 3, 1, 3, 6], 4, 12),
    ];
    for (nums, k, expected) in cases {
        let result = Solution::count_non_decreasing_subarrays(nums, k);
        assert_eq!(result, expected);
    }
}
