impl Solution {
    // IDEA: keep track of the last seen index of each character. Start looking for substrings
    // using the index AFTER the last seen index of the current character.
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut longest = 0;
        let mut start = 0;
        let mut last_seen: std::collections::HashMap<char, usize> =
            std::collections::HashMap::new();

        for (idx, c) in s.chars().enumerate() {
            // if the character is in the hash map, get the index of the its last occurence from
            // the hash map
            if let Some(last_seen_idx) = last_seen.get(&c) {
                // only run if the last seen index is greater than or equal to the current start
                if last_seen_idx >= &start {
                    // the longest must be the max between the current longest and the current index
                    // MINUS the current "starting" index
                    longest = longest.max(idx - start);
                    start = last_seen_idx + 1; // update the new start value
                }
            }

            // always insert/update the new last occurence of the character into the hash map
            last_seen.insert(c, idx);
        }

        // we have to take the max in case the longest string is the last substring (at the end)
        longest.max(s.len() - start) as i32
    }
}
