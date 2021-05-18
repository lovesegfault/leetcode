pub fn length_of_longest_substring(s: String) -> i32 {
    use std::collections::HashMap;

    let sb = s.as_bytes();

    let mut char_map: HashMap<u8, usize> = HashMap::with_capacity(sb.len());
    let mut start = 0;
    let mut longest = 0;

    for end in 0..sb.len() {
        // if this is a repeated character
        if char_map.contains_key(&sb[end]) {
            // then we update the start of our window to be where we last saw this character
            start = start.max(char_map[&sb[end]] + 1);
        }
        longest = longest.max(end - start + 1);
        char_map.insert(sb[end], end);
    }

    longest as i32
}

fn shim(s: &str) -> i32 {
    length_of_longest_substring(s.to_string())
}

fn main() {
    assert_eq!(shim("abcabcbb"), 3);
    assert_eq!(shim("bbbbb"), 1);
    assert_eq!(shim("pwwkew"), 3);
    assert_eq!(shim(""), 0);
}
