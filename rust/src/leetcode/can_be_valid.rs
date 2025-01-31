pub struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn can_be_valid(s: String, locked: String) -> bool {
        Self::can_be_valid_v2(s, locked)
    }
    #[allow(dead_code)]
    pub fn can_be_valid_v2(s: String, locked: String) -> bool {
        if s.len() % 2 == 1 {
            return false;
        }
        let locked = locked.into_bytes();
        let s = s.into_bytes();
        let mut opens = Vec::new();
        let mut unlocked = Vec::new();
        for (i, l) in locked.into_iter().enumerate() {
            if l == b'0' {
                unlocked.push(i);
            } else if s[i] == b'(' {
                opens.push(i);
            } else if opens.pop().or_else(|| unlocked.pop()).is_none() {
                return false;
            }
        }
        while opens.last().copied() < unlocked.pop() {
            opens.pop();
        }
        opens.is_empty()
    }
    #[allow(dead_code)]
    pub fn can_be_valid_v1(s: String, locked: String) -> bool {
        if s.len() & 1 == 1 {
            return false;
        }
        let mut left = 0;
        for (c, l) in s.chars().zip(locked.chars()) {
            if c == '(' || l == '0' {
                left += 1;
            } else if c == ')' {
                left -= 1;
                if left < 0 {
                    return false;
                }
            }
        }
        let mut right = 0;
        for (c, l) in s.chars().rev().zip(locked.chars().rev()) {
            if c == ')' || l == '0' {
                right += 1;
            } else if c == '(' {
                right -= 1;
                if right < 0 {
                    return false;
                }
            }
        }
        true
    }
}

