struct Solution;

impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let first_idx = nums.iter().position(|&x| x == target);
        let last_idx = nums.iter().rposition(|&x| x == target);

        match (first_idx, last_idx) {
            (Some(first), Some(last)) => vec![first as i32, last as i32],
            _ => vec![-1, -1],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            Solution::search_range(vec![5, 7, 7, 8, 8, 10], 8),
            vec![3, 4]
        );
        assert_eq!(
            Solution::search_range(vec![5, 7, 7, 8, 8, 10], 6),
            vec![-1, -1]
        );
        assert_eq!(Solution::search_range(vec![], 0), vec![-1, -1]);
    }
}
