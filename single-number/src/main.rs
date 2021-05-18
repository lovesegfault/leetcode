struct Solution;

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        use std::collections::HashSet;
        let mut seen: HashSet<i32> = HashSet::with_capacity((nums.len() / 2) + 1);
        for n in nums {
            if !seen.remove(&n) {
                seen.insert(n);
            }
        }
        seen.into_iter().next().unwrap()
    }
}

fn main() {
    println!("Hello, world!");
}
