struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack: Vec<char> = Vec::with_capacity(s.len());
        for c in s.chars() {
            match c {
                '(' | '[' | '{' => stack.push(c),
                _ => {
                    let wanted = match c {
                        ')' => '(',
                        ']' => '[',
                        '}' => '{',
                        _ => unreachable!(),
                    };
                    if stack.last() == Some(&wanted) {
                        stack.pop();
                    } else {
                        return false;
                    }
                }
            }
        }
        stack.len() == 0
    }
}

fn main() {
    println!("Hello, world!");
}
