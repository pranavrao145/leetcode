impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        }

        let mut q: Vec<i32> = Vec::new();
        let mut temp: i32 = x;

        while temp > 0 {
            q.push(temp % 10);
            temp /= 10;
        }

        loop {
            if q.len() <= 1 {
                // if it's 0 or 1, it is a palindrome
                return true;
            }

            // get first and last digits
            let first = q.remove(0);
            let last = q.pop().unwrap();

            // if they're not equal
            if first != last {
                return false; // immediately return false
            }
        }
    }
}
