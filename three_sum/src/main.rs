struct Solution;

impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        use std::collections::HashSet;

        if nums.len() < 3 {
            return vec![];
        }

        let mut set = HashSet::new();

        // nlogn
        nums.sort();

        for i in 0..(nums.len() - 2) {
            let a = nums[i];
            let mut left = i + 1;
            let mut right = nums.len() - 1;
            while left < right {
                let b = nums[left];
                let c = nums[right];
                if a + b + c == 0 {
                    set.insert(vec![a, b, c]);
                }
                if a + b + c < 0 {
                    left += 1;
                } else {
                    right -= 1;
                }
            }
        }
        // Need to own the elements of set
        set.iter().cloned().collect()
    }
}

fn main() {
    println!("Hello, world!");
}
