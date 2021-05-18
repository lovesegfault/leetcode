struct Solution;

impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let mut iter = s.chars().skip_while(char::is_ascii_whitespace).peekable();
        let (sign, clamp) = match iter.peek() {
            Some('+') => {
                iter.next();
                (1, i32::MAX)
            }
            Some('-') => {
                iter.next();
                (-1, i32::MIN)
            }
            _ => (1, i32::MAX),
        };

        iter.take_while(char::is_ascii_digit)
            .filter_map(|c| c.to_digit(10))
            .map(|d| d as i32 * sign)
            .try_fold(0i32, |acc, val| {
                acc.checked_mul(10).and_then(|base| base.checked_add(val))
            })
            .unwrap_or(clamp)
    }
}

fn main() {
    assert_eq!(Solution::my_atoi("".to_string()), 0);
    assert_eq!(Solution::my_atoi("42".to_string()), 42);
    assert_eq!(Solution::my_atoi("   -42".to_string()), -42);
    assert_eq!(Solution::my_atoi("4193 with words".to_string()), 4193);
    assert_eq!(Solution::my_atoi("words and 987".to_string()), 0);
    assert_eq!(Solution::my_atoi("-91283472332".to_string()), i32::MIN);
    assert_eq!(Solution::my_atoi("  1000000012".to_string()), 1000000012);
    assert_eq!(Solution::my_atoi("".to_string()), 0);
}
