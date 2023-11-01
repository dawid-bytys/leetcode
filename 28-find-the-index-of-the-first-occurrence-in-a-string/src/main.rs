struct Solution;

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        if needle.len() > haystack.len() {
            return -1;
        }

        for i in 0..haystack.len() - needle.len() + 1 {
            if &haystack[i..i + needle.len()] == needle {
                return i as i32;
            }
        }

        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::str_str("hello".to_string(), "ll".to_string()), 2);
        assert_eq!(
            Solution::str_str("aaaaa".to_string(), "bba".to_string()),
            -1
        );
        assert_eq!(Solution::str_str("".to_string(), "".to_string()), 0);
    }
}
