impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        {
            let mut ret: i32 = 0; // start at 0
            let chars: Vec<char> = s.chars().collect();

            let len = chars.len();

            for i in 0..len {
                match chars[i] {
                    'V' => ret += 5,
                    'L' => ret += 50,
                    'D' => ret += 500,
                    'M' => ret += 1000,
                    'I' => {
                        if i < len - 1 {
                            match chars[i + 1] {
                                'V' => ret -= 1,
                                'X' => ret -= 1,
                                _ => ret += 1,
                            }
                        } else {
                            ret += 1;
                        }
                    }
                    'X' => {
                        if i < len - 1 {
                            match chars[i + 1] {
                                'L' => ret -= 10,
                                'C' => ret -= 10,
                                _ => ret += 10,
                            }
                        } else {
                            ret += 10;
                        }
                    }
                    'C' => {
                        if i < len - 1 {
                            match chars[i + 1] {
                                'D' => ret -= 100,
                                'M' => ret -= 100,
                                _ => ret += 100,
                            }
                        } else {
                            ret += 100;
                        }
                    }
                    _ => {}
                }
            }

            return ret;
        }
    }
}
