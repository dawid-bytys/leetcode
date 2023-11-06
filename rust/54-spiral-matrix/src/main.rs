struct Solution;

impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let mut traversal: Vec<i32> = Vec::new();
        if matrix.is_empty() {
            return traversal;
        }

        let mut row_start = 0;
        let mut col_start = 0;
        let mut row_end = matrix.len();
        let mut col_end = matrix[0].len();

        while row_start < row_end && col_start < col_end {
            for col in col_start..col_end {
                traversal.push(matrix[row_start][col]);
            }
            row_start += 1;

            for row in row_start..row_end {
                traversal.push(matrix[row][col_end - 1]);
            }
            col_end -= 1;

            if row_start < row_end {
                for col in (col_start..col_end).rev() {
                    traversal.push(matrix[row_end - 1][col]);
                }
                row_end -= 1;
            }

            if col_start < col_end {
                for row in (row_start..row_end).rev() {
                    traversal.push(matrix[row][col_start]);
                }
                col_start += 1;
            }
        }

        traversal
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            Solution::spiral_order(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]),
            vec![1, 2, 3, 6, 9, 8, 7, 4, 5]
        );

        assert_eq!(
            Solution::spiral_order(vec![
                vec![1, 2, 3, 4],
                vec![5, 6, 7, 8],
                vec![9, 10, 11, 12]
            ]),
            vec![1, 2, 3, 4, 8, 12, 11, 10, 9, 5, 6, 7]
        );

        assert_eq!(
            Solution::spiral_order(vec![
                vec![1, 2, 3, 4],
                vec![5, 6, 7, 8],
                vec![9, 10, 11, 12],
                vec![13, 14, 15, 16]
            ]),
            vec![1, 2, 3, 4, 8, 12, 16, 15, 14, 13, 9, 5, 6, 7, 11, 10]
        );
    }
}
