struct Solution;

impl Solution {
    // brute force
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut out = Vec::with_capacity(2);
        for i in 0..nums.len() {
            let a = nums[i];
            for j in (i + 1)..nums.len() {
                let b = nums[j];
                if a + b == target {
                    out.push(i as i32);
                    out.push(j as i32);
                    return out;
                }
            }
        }
        out
    }

    // sorting
    pub fn two_sum2(nums: Vec<i32>, target: i32) -> Vec<i32> {
        use std::collections::HashMap;
        let mut res: Vec<i32> = Vec::with_capacity(2);
        let mut map: HashMap<i32, i32> = HashMap::with_capacity(nums.len());
        for (b, num) in nums.iter().enumerate() {
            if let Some(a) = map.get(num) {
                res.push(*a);
                res.push(b as i32);
                break;
            }
            map.insert(target - num, b as i32);
        }
        res
    }
}

fn main() {
    let nums = vec![2, 7, 11, 15];
    dbg!(Solution::two_sum2(nums, 9));

    let nums = vec![3, 2, 4];
    dbg!(Solution::two_sum2(nums, 6));

    let nums = vec![3, 3];
    dbg!(Solution::two_sum2(nums, 6));
}
