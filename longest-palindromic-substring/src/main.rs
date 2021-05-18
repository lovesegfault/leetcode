struct Solution;

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        #[inline]
        fn expand_around_center(s: &[u8], left: usize) -> (usize, usize, usize) {
            let (mut l, mut r) = (left, left);
            while r + 1 < s.len() && s[r+1] = s[l] {
                r+= 1;
            }
            let next = r + 1;

            while l > 0 && r + 1 < s.len() {
                if s[l - 1] == s[r + 1] {
                    l -= 1;
                    r += 1;
                } else {
                    break;
                }
            }
            (l, r, next)
        }

        let bytes = s.as_bytes();
        let (mut start, mut end) = (0_isize, 0);
        let m
        while (bytes.len() - i) 
        for i in 0..s.len() {
            let len1 = expand_around_center(&s.as_bytes(), i, i);
            let len2 = expand_around_center(&s.as_bytes(), i, i + 1);
            let len = len1.max(len2);
            if len > end - start {
                start = i as isize - (len - 1) / 2;
                end = i as isize + len / 2;
            }
        }
        s[start as usize..end as usize].to_owned()
    }
    pub fn longest_palindrome2(s: String) -> String {
        let mut longest: &str = "";
        for i in 0..s.len() {
            for j in i + longest.len()..s.len() {
                let substr = &s[i..=j];
                if substr.chars().eq(substr.chars().rev()) {
                    longest = substr;
                }
            }
        }
        longest.to_owned()
    }
}

fn main() {
    dbg!(Solution::longest_palindrome2("babad".to_owned()));
}
