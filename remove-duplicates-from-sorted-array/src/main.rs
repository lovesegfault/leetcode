struct Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut i = 0;
        while let Some(e) = nums.get(i) {
            // if we find a dupe next
            if Some(e) == nums.get(i + 1) {
                // remove it and continue loop from where we are.
                nums.remove(i + 1);
            } else {
                i += 1;
            }
        }
        nums.len() as i32
    }
}

fn main() {
    let mut a = vec![1, 1, 2];
    eprintln!("a = {:?}", a);
    Solution::remove_duplicates(&mut a);
    eprintln!("a = {:?}", a);

    let mut b = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
    eprintln!("a = {:?}", b);
    Solution::remove_duplicates(&mut b);
    eprintln!("a = {:?}", b);
}
