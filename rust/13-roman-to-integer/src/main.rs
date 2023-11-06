struct Solution;

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        fn char_to_value(c: char) -> i32 {
            match c {
                'I' => 1,
                'V' => 5,
                'X' => 10,
                'L' => 50,
                'C' => 100,
                'D' => 500,
                'M' => 1000,
                _ => panic!("Invalid Roman numeral"),
            }
        }

        let mut result = 0;
        let mut prev_value = 0;
        let chars = s.chars().collect::<Vec<char>>();
        let len = chars.len();

        for i in (0..len).rev() {
            let value = char_to_value(chars[i]);
            if value < prev_value {
                result -= value;
            } else {
                result += value;
            }
            prev_value = value;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::roman_to_int("III".to_string()), 3);
        assert_eq!(Solution::roman_to_int("IV".to_string()), 4);
        assert_eq!(Solution::roman_to_int("IX".to_string()), 9);
        assert_eq!(Solution::roman_to_int("LVIII".to_string()), 58);
        assert_eq!(Solution::roman_to_int("MCMXCIV".to_string()), 1994);
    }
}
