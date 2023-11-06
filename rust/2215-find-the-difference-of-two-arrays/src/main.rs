use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn find_difference(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<Vec<i32>> {
        let set_one: HashSet<i32> = nums1.into_iter().collect();
        let set_two: HashSet<i32> = nums2.into_iter().collect();

        let difference_one: Vec<i32> = set_one.difference(&set_two).cloned().collect();
        let difference_two: Vec<i32> = set_two.difference(&set_one).cloned().collect();

        vec![difference_one, difference_two]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            Solution::find_difference(vec![1, 2, 3], vec![1, 2, 4]),
            vec![vec![3], vec![4]]
        );
        assert_eq!(
            Solution::find_difference(vec![1, 2, 3], vec![1, 2, 3]),
            vec![vec![], vec![]]
        );
        assert_eq!(
            Solution::find_difference(vec![1, 2, 3], vec![1, 2, 3, 4]),
            vec![vec![], vec![4]]
        );
        assert_eq!(
            Solution::find_difference(vec![1, 2, 3, 4], vec![1, 2, 3]),
            vec![vec![4], vec![]]
        );
    }
}
