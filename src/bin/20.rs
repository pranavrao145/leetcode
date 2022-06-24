impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack: Vec<char> = Vec::new();

        for char in s.chars() {
            match char {
                '[' | '(' | '{' => stack.push(char),
                ']' => {
                    if stack.is_empty() || stack.pop().unwrap() != '[' {
                        return false;
                    }
                }
                '}' => {
                    if stack.is_empty() || stack.pop().unwrap() != '{' {
                        return false;
                    }
                }
                ')' => {
                    if stack.is_empty() || stack.pop().unwrap() != '(' {
                        return false;
                    }
                }
                _ => {}
            }
        }

        return stack.is_empty();
    }
}
