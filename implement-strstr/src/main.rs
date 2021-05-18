struct Solution;

impl Solution {
    // modified Knuth-Morris-Pratt for only finding the first occurence
    pub fn str_str(haystack: String, needle: String) -> i32 {
        if needle.len() == 0 {
            return 0;
        }

        let table = Self::build_kmp_table(&needle);

        let w = needle.as_bytes();
        let mut j = 0;

        let s = haystack.as_bytes();
        let mut k = 0;

        while j < s.len() {
            if w[k] == s[j] {
                j += 1;
                k += 1;
                if k == w.len() {
                    return (j - k) as i32;
                }
            } else {
                if table[k] < 0 {
                    k = 0;
                    j += 1;
                } else {
                    k = table[k] as usize;
                }
            }
        }

        -1
    }

    #[inline]
    fn build_kmp_table(needle: &str) -> Vec<i32> {
        let b = needle.as_bytes();
        let mut table = vec![0; b.len() + 1];
        table[0] = -1;

        let mut pos = 1;
        let mut cnd: i32 = 0;

        while pos < b.len() {
            if b[pos] == b[cnd as usize] {
                table[pos] = table[cnd as usize];
            } else {
                table[pos] = cnd as i32;
                while cnd >= 0 && b[pos] != b[cnd as usize] {
                    cnd = table[cnd as usize];
                }
            }
            pos += 1;
            cnd += 1;
        }

        table[pos] = cnd;

        table
    }
}

fn main() {
    dbg!(Solution::str_str("hello".to_string(), "ll".to_string()));
    dbg!(Solution::str_str("".to_string(), "".to_string()));
    dbg!(Solution::str_str("aaaaa".to_string(), "bba".to_string()));
}
