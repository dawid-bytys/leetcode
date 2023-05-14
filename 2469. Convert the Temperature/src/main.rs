struct Solution;

impl Solution {
    pub fn convert_temperature(celsius: f64) -> Vec<f64> {
        let kelvin: f64 = celsius + 273.15;
        let fahrenheit: f64 = celsius * 1.80 + 32.00;
        vec![kelvin, fahrenheit]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::convert_temperature(0.0), vec![273.15, 32.0]);
        assert_eq!(Solution::convert_temperature(100.0), vec![373.15, 212.0]);
    }
}
