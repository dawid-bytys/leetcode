struct Solution;

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut maximum_sum = nums[0];
        let mut current_sum = maximum_sum;

        for i in 1..nums.len() {
            current_sum = std::cmp::max(nums[i], current_sum + nums[i]);
            maximum_sum = std::cmp::max(maximum_sum, current_sum);
        }

        return maximum_sum;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            Solution::max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]),
            6
        );
        assert_eq!(Solution::max_sub_array(vec![1]), 1);
        assert_eq!(Solution::max_sub_array(vec![5, 4, -1, 7, 8]), 23);
    }
}
