struct Solution;

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        use std::collections::HashSet;
        let s: HashSet<_> = nums.iter().collect();
        s.len() != nums.len()
    }

    pub fn contains_duplicate2(mut nums: Vec<i32>) -> bool {
        nums.sort();
        for i in 1..nums.len() {
            if nums[i] == nums[i - 1] {
                return true;
            }
        }
        false
    }
}

fn main() {
    println!("Hello, world!");
}
