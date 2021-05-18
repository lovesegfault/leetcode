struct Solution;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }

        let s = s.into_bytes();
        let t = t.into_bytes();

        let l = s.len();
        let mut counts = [0; 26];
        for i in 0..l {
            let s_idx = s[i] as usize - 97;
            let t_idx = t[i] as usize - 97;

            counts[s_idx] += 1;
            counts[t_idx] -= 1;
        }

        counts.iter().all(|&c| c == 0)
    }
}

fn main() {
    println!("Hello, world!");
}
