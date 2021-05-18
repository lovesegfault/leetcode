struct Solution;

impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let l = nums.len() - 1;
        let mut i = 0;
        let mut j = 0;

        while i <= l {
            // bring j up to speed if need be
            if j < i {
                j = i;
            }

            // find the next non-zero element after i
            while j < l && nums[j] == 0 {
                j += 1;
            }

            if nums[i] == 0 {
                // swap our zero with the first non-zero after it.
                nums.swap(i, j);
            }

            i += 1;
        }
    }
}

fn main() {
    let mut nums = vec![1, 0, 0];
    println!("{:?}", nums);
    Solution::move_zeroes(&mut nums);
    println!("{:?}", nums);
}
