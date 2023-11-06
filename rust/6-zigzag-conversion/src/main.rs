struct Solution;

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows == 1 || s.len() <= num_rows as usize {
            return s;
        }

        let mut rows = vec![String::new(); num_rows as usize];
        let mut current_row = 0;
        let mut direction = -1;

        for ch in s.chars() {
            rows[current_row].push(ch);
            if current_row == 0 || current_row == (num_rows - 1) as usize {
                direction *= -1;
            }
            current_row = (current_row as i32 + direction) as usize;
        }

        rows.join("")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            Solution::convert("PAYPALISHIRING".to_string(), 3),
            "PAHNAPLSIIGYIR".to_string()
        );
        assert_eq!(
            Solution::convert("PAYPALISHIRING".to_string(), 4),
            "PINALSIGYAHRPI".to_string()
        );
        assert_eq!(Solution::convert("A".to_string(), 1), "A".to_string());
    }
}
