struct Solution;

impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut digits_copy = digits.clone();

        for i in (0..digits_copy.len()).rev() {
            if digits_copy[i] != 9 {
                digits_copy[i] += 1;
                return digits_copy;
            }

            digits_copy[i] = 0;
        }

        digits_copy.insert(0, 1);
        return digits_copy;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::plus_one(vec![1, 2, 3]), vec![1, 2, 4]);
        assert_eq!(Solution::plus_one(vec![4, 3, 2, 1]), vec![4, 3, 2, 2]);
        assert_eq!(Solution::plus_one(vec![4, 3, 2, 1]), vec![4, 3, 2, 2]);
        assert_eq!(Solution::plus_one(vec![0]), vec![1]);
    }
}
