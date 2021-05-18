struct Solution;

impl Solution {
    pub fn reverse_string(s: &mut Vec<char>) {
        // s.reverse();
        let l = s.len();
        for i in 0..(l / 2) {
            s.swap(i, l - i - 1);
        }
    }
}

fn main() {
    let mut s = vec!['H', 'a', 'n', 'n', 'a', 'h'];
    Solution::reverse_string(&mut s);
    dbg!(s);
}
