struct Solution;

impl Solution {
    pub fn smallest_even_multiple(n: i32) -> i32 {
        (n * 2).abs() / Solution::gcd(n, 2)
    }

    pub fn gcd(a: i32, b: i32) -> i32 {
        if b == 0 {
            return a;
        }

        Solution::gcd(b, a % b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::smallest_even_multiple(1), 2);
        assert_eq!(Solution::smallest_even_multiple(3), 6);
        assert_eq!(Solution::smallest_even_multiple(4), 4);
        assert_eq!(Solution::smallest_even_multiple(5), 10);
        assert_eq!(Solution::smallest_even_multiple(6), 6);
        assert_eq!(Solution::smallest_even_multiple(7), 14);
        assert_eq!(Solution::smallest_even_multiple(8), 8);
        assert_eq!(Solution::smallest_even_multiple(9), 18);
        assert_eq!(Solution::smallest_even_multiple(10), 10);
        assert_eq!(Solution::smallest_even_multiple(11), 22);
        assert_eq!(Solution::smallest_even_multiple(12), 12);
        assert_eq!(Solution::smallest_even_multiple(13), 26);
        assert_eq!(Solution::smallest_even_multiple(14), 14);
        assert_eq!(Solution::smallest_even_multiple(15), 30);
        assert_eq!(Solution::smallest_even_multiple(16), 16);
        assert_eq!(Solution::smallest_even_multiple(17), 34);
        assert_eq!(Solution::smallest_even_multiple(18), 18);
        assert_eq!(Solution::smallest_even_multiple(19), 38);
        assert_eq!(Solution::smallest_even_multiple(20), 20);
        assert_eq!(Solution::smallest_even_multiple(21), 42);
        assert_eq!(Solution::smallest_even_multiple(22), 22);
        assert_eq!(Solution::smallest_even_multiple(23), 46);
    }
}
