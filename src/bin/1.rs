use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut m: HashMap<_, _> = HashMap::new();
        let mut sol: Vec<i32> = Vec::new();

        for (i, j) in nums.iter().enumerate() {
            m.insert(j, i);
        }

        for (i, j) in nums.iter().enumerate() {
            if m.contains_key(&(target - j)) && (m.get(&(target - j))).unwrap() != &i {
                sol.push(i as i32);
                sol.push(*m.get(&(target - j)).unwrap() as i32);
                return sol;
            }
        }

        return sol;
    }
}
