struct Solution;

impl Solution {
    pub fn reverse(mut x: i32) -> i32 {
        let mut rev: i32 = 0;
        while x != 0 {
            match rev.checked_mul(10).and_then(|r| r.checked_add(x % 10)) {
                Some(ok) => {
                    rev = ok;
                }
                None => return 0,
            }
            x /= 10;
        }

        rev
    }
}

fn main() {
    println!("Hello, world!");
}
