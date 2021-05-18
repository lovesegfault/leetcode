struct Solution;

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let s = s.into_bytes();
        let mut start = 0;
        let mut end = s.len() - 1;

        while start < end {
            if !s[start].is_ascii_alphanumeric() {
                start += 1;
                continue;
            }

            if !s[end].is_ascii_alphanumeric() {
                end -= 1;
                continue;
            }

            if s[start].to_ascii_lowercase() != s[end].to_ascii_lowercase() {
                return false;
            }

            start += 1;
            end -= 1;
        }

        true
    }

    pub fn is_palindrome2(s: String) -> bool {
        let iter = s
            .chars()
            .filter(char::is_ascii_alphanumeric)
            .map(|c| c.to_ascii_lowercase());

        iter.clone().eq(iter.rev())
    }
}

fn main() {
    println!("Hello, world!");
}
