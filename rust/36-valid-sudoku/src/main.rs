use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut set = HashSet::new();
        for i in 0..9 {
            if !Self::is_valid_row(&board, i, &mut set)
                || !Self::is_valid_column(&board, i, &mut set)
                || !Self::is_valid_box(&board, i % 3 * 3, i / 3 * 3, &mut set)
            {
                return false;
            }
        }
        true
    }

    fn is_valid_row(board: &Vec<Vec<char>>, row: usize, set: &mut HashSet<char>) -> bool {
        set.clear();
        for col in 0..9 {
            let val = board[row][col];
            if val == '.' {
                continue;
            }
            if set.contains(&val) {
                return false;
            }
            set.insert(val);
        }
        true
    }

    fn is_valid_column(board: &Vec<Vec<char>>, col: usize, set: &mut HashSet<char>) -> bool {
        set.clear();
        for row in 0..9 {
            let val = board[row][col];
            if val == '.' {
                continue;
            }
            if set.contains(&val) {
                return false;
            }
            set.insert(val);
        }
        true
    }

    fn is_valid_box(
        board: &Vec<Vec<char>>,
        start_row: usize,
        start_col: usize,
        set: &mut HashSet<char>,
    ) -> bool {
        set.clear();
        for row in start_row..start_row + 3 {
            for col in start_col..start_col + 3 {
                let val = board[row][col];
                if val == '.' {
                    continue;
                }
                if set.contains(&val) {
                    return false;
                }
                set.insert(val);
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let board = vec![
            vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ];
        assert_eq!(Solution::is_valid_sudoku(board), true);
    }
}
