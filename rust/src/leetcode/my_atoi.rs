
pub struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn my_atoi(s: String) -> i32 {
        const BOUND: i32 = i32::MAX / 10;
        const ZERO: i32 = '0' as i32;
        let mut sign = 0;
        let mut r = 0;
        for c in s.bytes().into_iter() {
            match c as char {
                ' ' | '+' | '-' => {
                    if sign != 0 {
                        break;
                    }
                    sign = match c as char {
                        '+' => 1,
                        '-' => -1,
                        _ => 0,
                    }
                }
                '0'..='9' => {
                    let digit = c as i32 - ZERO;
                    if sign == 0 {
                        sign = 1;
                    }
                    if r > BOUND || (r == BOUND && digit > 7) {
                        return if sign > 0 { i32::MAX } else { i32::MIN };
                    }
                    r = r * 10 + digit;
                }
                _ => {
                    break;
                }
            }
        }
        r * sign
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_my_atoi() {
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
}
