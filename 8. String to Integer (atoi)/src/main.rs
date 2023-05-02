struct Solution;

impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let mut iter = s.chars().peekable();
        let mut result: i64 = 0;
        let mut sign = 1;

        while iter.peek() == Some(&' ') {
            iter.next();
        }

        match iter.peek() {
            Some(&'-') => {
                sign = -1;
                iter.next();
            }
            Some(&'+') => {
                iter.next();
            }
            _ => {}
        }

        while let Some(&c) = iter.peek() {
            if c.is_digit(10) {
                result = result * 10 + c.to_digit(10).unwrap() as i64;
                if result * sign > i32::MAX as i64 {
                    return i32::MAX;
                } else if result * sign < i32::MIN as i64 {
                    return i32::MIN;
                }
            } else {
                break;
            }
            iter.next();
        }

        (result * sign) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::my_atoi("42".to_string()), 42);
        assert_eq!(Solution::my_atoi("   -42".to_string()), -42);
        assert_eq!(Solution::my_atoi("4193 with words".to_string()), 4193);
        assert_eq!(Solution::my_atoi("words and 987".to_string()), 0);
        assert_eq!(Solution::my_atoi("-91283472332".to_string()), -2147483648);
    }
}
