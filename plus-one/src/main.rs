struct Solution;

impl Solution {
    pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
        let mut carry = 1;
        let mut i = digits.len() as isize - 1;

        while carry > 0 {
            if i < 0 {
                digits.insert(0, 0);
                i = 0;
            }

            let sum = digits[i as usize] + carry;

            digits[i as usize] = sum.rem_euclid(10);
            carry = sum / 10;

            i -= 1;
        }

        digits
    }
}

fn main() {
    dbg!(Solution::plus_one(vec![1, 2, 3]));
    dbg!(Solution::plus_one(vec![9]));
}
