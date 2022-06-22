impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut mut_str = s;
        let mut ret: i32 = 0;

        let letters: std::collections::HashMap<char, i32> = std::collections::HashMap::from([
            ('I', 1),
            ('V', 5),
            ('X', 10),
            ('L', 50),
            ('C', 100),
            ('D', 500),
            ('M', 1000),
        ]);

        let modifying_letters: std::collections::HashMap<char, char> =
            std::collections::HashMap::from([
                ('V', 'I'),
                ('X', 'I'),
                ('L', 'X'),
                ('C', 'X'),
                ('D', 'C'),
                ('M', 'C'),
            ]);

        while mut_str.len() > 0 {
            let curr_char = mut_str.chars().nth_back(0).unwrap();
            let curr_char_val = letters.get(&curr_char).unwrap();

            // two or more characters
            if mut_str.len() > 1 {
                let prev_char = mut_str.chars().nth_back(1).unwrap();
                let prev_char_val = letters.get(&prev_char).unwrap();
                let modifier = *modifying_letters.get(&curr_char).unwrap_or(&'0');

                // if the prev character is a modifying character
                if modifier == prev_char {
                    ret += curr_char_val - prev_char_val;
                    mut_str = mut_str.chars().take(mut_str.len() - 2).collect(); // chop off last two
                    continue;
                }
            }

            // if it's the only char or there is no modifier
            ret += curr_char_val;
            mut_str = mut_str.chars().take(mut_str.len() - 1).collect(); // chop off last one
        }

        return ret;
    }
}
