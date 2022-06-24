impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        if strs.len() == 0 {
            return "".to_string();
        }

        let mut prefix = strs[0].clone(); // start with the entire first string as the prefix

        for i in 1..strs.len() {
            while strs[i].find(&prefix).unwrap_or_else(|| usize::MAX) != 0 {
                prefix = prefix.chars().take(prefix.len() - 1).collect();
                if prefix.is_empty() {
                    return "".to_string();
                }
            }
        }

        return prefix;
    }
}
