use std::collections::HashMap;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut hash: HashMap<String, Vec<String>> = HashMap::new();

        for str in strs {
            let mut sorted_str: Vec<char> = str.chars().collect();
            sorted_str.sort();
            let sorted_str: String = sorted_str.iter().collect();

            hash.entry(sorted_str)
                .or_insert_with(Vec::new)
                .push(str);
        }

        hash.into_values().collect()
    }
}
