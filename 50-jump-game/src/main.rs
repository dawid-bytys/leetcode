struct Solution;

impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let mut current_goal = nums.len() - 1;

        for i in (0..nums.len() - 1).rev() {
            if i + nums[i] as usize >= current_goal {
                current_goal = i;
            }
        }

        if current_goal == 0 {
            return true;
        }

        return false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::can_jump(vec![2, 3, 1, 1, 4]), true);
        assert_eq!(Solution::can_jump(vec![3, 2, 1, 0, 4]), false);
        assert_eq!(Solution::can_jump(vec![0]), true);
    }
}
