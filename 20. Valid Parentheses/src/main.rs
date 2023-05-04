struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = Vec::new();

        for ch in s.chars() {
            match ch {
                '{' => stack.push('}'),
                '(' => stack.push(')'),
                '[' => stack.push(']'),
                '}' | ')' | ']' if Some(ch) != stack.pop() => return false,
                _ => continue,
            }
        }

        stack.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::is_valid("()".to_string()), true);
        assert_eq!(Solution::is_valid("()[]{}".to_string()), true);
        assert_eq!(Solution::is_valid("(]".to_string()), false);
        assert_eq!(Solution::is_valid("([)]".to_string()), false);
        assert_eq!(Solution::is_valid("{[]}".to_string()), true);
    }
}
