struct Solution;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut anagram_map: std::collections::HashMap<String, Vec<String>> =
            std::collections::HashMap::new();

        for word in strs {
            let mut chars: Vec<char> = word.chars().collect();
            chars.sort();
            let sorted_word: String = chars.into_iter().collect();

            match anagram_map.get_mut(&sorted_word) {
                Some(anagrams) => anagrams.push(word),
                None => {
                    anagram_map.insert(sorted_word, vec![word]);
                }
            }
        }

        anagram_map.into_iter().map(|(_, v)| v).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input: Vec<String> = vec![
            "eat".to_string(),
            "tea".to_string(),
            "tan".to_string(),
            "ate".to_string(),
            "nat".to_string(),
            "bat".to_string(),
        ];
        let expected: Vec<Vec<String>> = vec![
            vec!["eat".to_string(), "tea".to_string(), "ate".to_string()],
            vec!["tan".to_string(), "nat".to_string()],
            vec!["bat".to_string()],
        ];
        assert_eq!(Solution::group_anagrams(input), expected);
    }
}
