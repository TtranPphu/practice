#![allow(dead_code)]

#[test]
fn can_be_valid() {
  use leetcode::can_be_valid::Solution;
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

// #[test]
fn count_non_decreasing_subarrays() {
  use leetcode::cnds::Solution;
  let cases = [
    (vec![6, 3, 1, 2, 4, 4], 7, 17),
    (vec![6, 3, 1, 3, 6], 4, 12),
  ];
  for (nums, k, expected) in cases {
    let result = Solution::count_non_decreasing_subarrays(nums, k);
    assert_eq!(result, expected);
  }
}

#[test]
fn median_sliding_window() {
  use leetcode::median_sliding_window::Solution;
  let cases = [
    (
      vec![1, 3, -1, -3, 5, 3, 6, 7],
      3,
      vec![1.0, -1.0, -1.0, 3.0, 5.0, 6.0],
    ),
    (
      vec![1, 2, 3, 4, 2, 3, 1, 4, 2],
      3,
      vec![2.0, 3.0, 3.0, 3.0, 2.0, 3.0, 2.0],
    ),
    (vec![2147483647, 2147483647], 2, vec![2147483647.0]),
  ];
  for (nums, k, expected) in cases {
    let result = Solution::median_sliding_window(nums, k);
    assert_eq!(result, expected);
  }
}

#[test]
fn merge_two_lists() {
  use leetcode::merge_two_lists::{ListNode, Solution};
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

#[test]
fn monkey_move() {
  use leetcode::monkey_move::Solution;
  assert_eq!(Solution::monkey_move(3), 6);
  assert_eq!(Solution::monkey_move(4), 14);
  assert_eq!(Solution::monkey_move(55), 766762394);
  assert_eq!(Solution::monkey_move(500000004), 0);
}

#[test]
fn my_atoi() {
  use leetcode::my_atoi::Solution;
  let cases = [
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

#[test]
fn my_pow() {
  use assert_approx_eq::assert_approx_eq;
  use leetcode::my_pow::Solution;
  let cases = [
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

#[test]
fn reverse() {
  use leetcode::reverse::Solution;
  assert_eq!(Solution::reverse(123), 321);
  assert_eq!(Solution::reverse(-123), -321);
  assert_eq!(Solution::reverse(120), 21);
  assert_eq!(Solution::reverse(0), 0);
  assert_eq!(Solution::reverse(1534236469), 0);
  assert_eq!(Solution::reverse(-2147483648), 0);
}

#[test]
fn solve_n_queens() {
  use leetcode::solve_n_queens::Solution;
  let cases = [
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

#[test]
fn two_sum() {
  use leetcode::two_sum::Solution;
  let cases = [
    (vec![2, 7, 11, 15], 9, vec![0, 1]),
    (vec![3, 2, 4], 6, vec![1, 2]),
    (vec![3, 3], 6, vec![0, 1]),
  ];
  for (nums, target, expected) in cases {
    let result = Solution::two_sum(nums, target);
    assert_eq!(result, expected);
  }
}

#[test]
fn minimize_xor() {
  use leetcode::xor::minimize_xor::Solution;
  let cases = [(3, 5, 3), (1, 12, 3), (25, 72, 24)];
  for (num1, num2, expected) in cases {
    let result = Solution::minimize_xor(num1, num2);
    assert_eq!(result, expected);
  }
}

#[test]
fn xor_all_nums() {
  use leetcode::xor::xor_all_nums::Solution;
  let cases = [
    (vec![2, 1, 3], vec![10, 2, 5, 0], 13),
    (vec![1, 2], vec![3, 4], 0),
    (
      vec![
        365, 744, 407, 833, 993, 455, 904, 808, 116, 853, 121, 380, 137, 53, 846, 50, 338, 460,
        630, 276, 509, 48, 530, 440, 975, 434, 556, 875, 795, 317, 749, 164, 736, 554, 887, 455,
        706, 311, 682, 548, 56, 632, 818, 538, 681, 312, 837, 833, 565, 842, 725, 27, 330, 0, 572,
        701, 343, 967, 287, 959, 113, 136, 538, 752, 454, 22, 805, 421, 281, 906, 119, 51, 152,
        632, 848, 158, 19, 997, 184, 447, 38, 515, 440, 540, 195, 743, 939, 476, 860, 77, 66,
      ],
      vec![
        537, 817, 983, 527, 547, 804, 300, 486, 96, 674, 654, 71, 465, 441, 675, 287, 749, 38, 501,
        967, 292, 460, 763, 611, 105, 27, 215, 658, 328, 37, 864, 581, 683, 499, 325, 884, 954,
        601, 86, 981, 926, 273, 586, 139, 246, 293, 107, 157, 635, 738, 693, 888, 598, 433, 860,
        165, 718, 502, 31, 164, 689, 604, 213,
      ],
      772,
    ),
  ];
  for (nums1, nums2, expected) in cases {
    let result = Solution::xor_all_nums(nums1, nums2);
    assert_eq!(result, expected);
  }
}

#[test]
fn does_valid_array_exist() {
  use leetcode::xor::does_valid_array_exist::Solution;
  let cases = [
    (vec![1, 1, 0], true),
    (vec![1, 1], true),
    (vec![1, 0], false),
  ];
  for (derived, expected) in cases {
    let result = Solution::does_valid_array_exist(derived);
    assert_eq!(result, expected);
  }
}
