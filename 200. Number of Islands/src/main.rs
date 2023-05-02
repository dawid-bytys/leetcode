struct Solution;

pub fn dfs(grid: &mut Vec<Vec<char>>, row: usize, col: usize) {
    let rows = grid.len();
    let cols = grid[0].len();

    if row >= rows || col >= cols || grid[row][col] == '0' {
        return;
    }

    grid[row][col] = '0';

    if row > 0 {
        dfs(grid, row - 1, col);
    }
    if col > 0 {
        dfs(grid, row, col - 1);
    }
    dfs(grid, row + 1, col);
    dfs(grid, row, col + 1);
}

impl Solution {
    pub fn num_islands(mut grid: Vec<Vec<char>>) -> i32 {
        let mut count = 0;

        for row in 0..grid.len() {
            for col in 0..grid[0].len() {
                if grid[row][col] == '1' {
                    count += 1;
                    dfs(&mut grid, row, col);
                }
            }
        }

        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            Solution::num_islands(vec![
                vec!['1', '1', '1', '1', '0'],
                vec!['1', '1', '0', '1', '0'],
                vec!['1', '1', '0', '0', '0'],
                vec!['0', '0', '0', '0', '0']
            ]),
            1
        );
        assert_eq!(
            Solution::num_islands(vec![
                vec!['1', '1', '0', '0', '0'],
                vec!['1', '1', '0', '0', '0'],
                vec!['0', '0', '1', '0', '0'],
                vec!['0', '0', '0', '1', '1']
            ]),
            3
        );
    }
}
