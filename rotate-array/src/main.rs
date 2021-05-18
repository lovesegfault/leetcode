struct Solution;

impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let len = nums.len() as i32;
        let k = len - k.rem_euclid(len);
        let left: Vec<_> = nums.drain(..k as usize).collect();
        nums.extend_from_slice(&left);
    }

    pub fn rotate2(nums: &mut Vec<i32>, k: i32) {
        let k = k as usize;
        let len = nums.len();

        for _ in 0..k {
            let rot_pos = len - 1;
            let tmp = nums[rot_pos];
            for j in (1..=rot_pos).rev() {
                nums[j] = nums[j - 1];
            }
            nums[0] = tmp;
        }
    }
}

fn main() {
    let mut nums = vec![1, 2, 3, 4, 5, 6, 7];
    Solution::rotate2(&mut nums, 3);
    dbg!(nums);
}
