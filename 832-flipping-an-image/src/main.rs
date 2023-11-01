struct Solution;

impl Solution {
    pub fn flip_and_invert_image(image: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        image
            .into_iter()
            .map(|mut row| {
                row.reverse();
                row.into_iter().map(|val| 1 - val).collect()
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flip_and_invert_image() {
        assert_eq!(
            Solution::flip_and_invert_image(vec![vec![1, 1, 0], vec![1, 0, 1], vec![0, 0, 0]]),
            vec![vec![1, 0, 0], vec![0, 1, 0], vec![1, 1, 1]]
        );
        assert_eq!(
            Solution::flip_and_invert_image(vec![
                vec![1, 1, 0, 0],
                vec![1, 0, 0, 1],
                vec![0, 1, 1, 1],
                vec![1, 0, 1, 0]
            ]),
            vec![
                vec![1, 1, 0, 0],
                vec![0, 1, 1, 0],
                vec![0, 0, 0, 1],
                vec![1, 0, 1, 0]
            ]
        );
    }
}
