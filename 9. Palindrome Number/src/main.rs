struct Solution;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        let mut reversed_number = 0;
        let mut temp = x;

        while temp > 0 {
            reversed_number = reversed_number * 10 + temp % 10;
            temp /= 10;
        }

        reversed_number == x
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::is_palindrome(121), true);
        assert_eq!(Solution::is_palindrome(-121), false);
        assert_eq!(Solution::is_palindrome(10), false);
        assert_eq!(Solution::is_palindrome(-101), false);
    }
}
