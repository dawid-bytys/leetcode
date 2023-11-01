struct Solution;

impl Solution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        if n == 0 || x == 1.0 {
            return 1.0;
        }

        if n == 1 {
            return x;
        }

        if n % 2 == 0 {
            let half = Solution::my_pow(x, n / 2);
            return half * half;
        }

        if n > 0 {
            return x * Solution::my_pow(x, n - 1);
        }

        1.0 / Solution::my_pow(x, -n)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::my_pow(2.0, 10), 1024.0);
        assert_eq!(Solution::my_pow(2.1, 3), 9.261);
        assert_eq!(Solution::my_pow(2.0, -2), 0.25);
    }
}
