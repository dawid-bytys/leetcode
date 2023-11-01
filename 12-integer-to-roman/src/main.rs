struct Solution;

impl Solution {
    pub fn int_to_roman(num: i32) -> String {
        let values = [
            (1000, "M"),
            (900, "CM"),
            (500, "D"),
            (400, "CD"),
            (100, "C"),
            (90, "XC"),
            (50, "L"),
            (40, "XL"),
            (10, "X"),
            (9, "IX"),
            (5, "V"),
            (4, "IV"),
            (1, "I"),
        ];

        let mut remaining = num;
        let mut roman = String::new();

        for (value, symbol) in values.iter() {
            while remaining >= *value {
                roman.push_str(symbol);
                remaining -= value;
            }
        }

        roman
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::int_to_roman(3), "III".to_string());
        assert_eq!(Solution::int_to_roman(4), "IV".to_string());
        assert_eq!(Solution::int_to_roman(9), "IX".to_string());
        assert_eq!(Solution::int_to_roman(58), "LVIII".to_string());
        assert_eq!(Solution::int_to_roman(1994), "MCMXCIV".to_string());
    }
}
