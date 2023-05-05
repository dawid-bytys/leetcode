struct Solution;

impl Solution {
    pub fn diagonal_sum(mat: Vec<Vec<i32>>) -> i32 {
        let mut elements: Vec<i32> = Vec::new();
        let n = mat.len();

        for i in 0..n {
            if i == n - i - 1 {
                elements.push(mat[i][i]);
            } else {
                elements.push(mat[i][i]);
                elements.push(mat[i][n - i - 1]);
            }
        }

        elements.iter().fold(0, |acc, &x| acc + x)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_diagonal_sum() {
        assert_eq!(
            Solution::diagonal_sum(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]),
            25
        );
        assert_eq!(
            Solution::diagonal_sum(vec![
                vec![1, 1, 1, 1],
                vec![1, 1, 1, 1],
                vec![1, 1, 1, 1],
                vec![1, 1, 1, 1]
            ]),
            8
        );
        assert_eq!(Solution::diagonal_sum(vec![vec![5]]), 5);
    }
}
