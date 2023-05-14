struct Solution;

impl Solution {
    pub fn subset_xor_sum(nums: Vec<i32>) -> i32 {
        let subsets = Solution::power_set(&nums);
        let mut sum = 0;

        for subset in subsets {
            let xor_total = subset.iter().fold(0, |acc, num| acc ^ num);
            sum += xor_total;
        }

        sum
    }

    pub fn power_set<T: Clone>(set: &[T]) -> Vec<Vec<T>> {
        let mut power_set = Vec::new();
        power_set.push(Vec::new());

        for item in set.iter() {
            let len = power_set.len();

            for i in 0..len {
                let mut subset = power_set[i].clone();
                subset.push(item.clone());
                power_set.push(subset);
            }
        }

        power_set
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::subset_xor_sum(vec![1, 3]), 6);
        assert_eq!(Solution::subset_xor_sum(vec![5, 1, 6]), 28);
        assert_eq!(Solution::subset_xor_sum(vec![3, 4, 5, 6, 7, 8]), 480);
    }
}
