struct Solution;

impl Solution {
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        let rows_count = board.len();
        let cols_count = board[0].len();
        let word_chars = word.chars().collect();
        let mut visited = vec![vec![false; cols_count]; rows_count];

        for i in 0..rows_count {
            for j in 0..cols_count {
                if Self::dfs(&board, &word_chars, 0, i, j, &mut visited) {
                    return true;
                }
            }
        }

        false
    }

    fn dfs(
        board: &Vec<Vec<char>>,
        word: &Vec<char>,
        idx: usize,
        x: usize,
        y: usize,
        visited: &mut Vec<Vec<bool>>,
    ) -> bool {
        if idx == word.len() {
            return true;
        }

        if x >= board.len() || y >= board[0].len() || visited[x][y] || board[x][y] != word[idx] {
            return false;
        }

        visited[x][y] = true;

        let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];
        for &(dx, dy) in directions.iter() {
            let new_x = (x as isize + dx) as usize;
            let new_y = (y as isize + dy) as usize;
            if Self::dfs(board, word, idx + 1, new_x, new_y, visited) {
                return true;
            }
        }

        visited[x][y] = false;
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exist() {
        assert_eq!(
            Solution::exist(
                vec![
                    vec!['A', 'B', 'C', 'E'],
                    vec!['S', 'F', 'C', 'S'],
                    vec!['A', 'D', 'E', 'E']
                ],
                "ABCCED".to_owned()
            ),
            true
        );
        assert_eq!(
            Solution::exist(
                vec![
                    vec!['A', 'B', 'C', 'E'],
                    vec!['S', 'F', 'C', 'S'],
                    vec!['A', 'D', 'E', 'E']
                ],
                "SEE".to_owned()
            ),
            true
        );
        assert_eq!(
            Solution::exist(
                vec![
                    vec!['A', 'B', 'C', 'E'],
                    vec!['S', 'F', 'C', 'S'],
                    vec!['A', 'D', 'E', 'E']
                ],
                "ABCB".to_owned()
            ),
            false
        );
    }
}
