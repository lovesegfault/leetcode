use std::collections::HashSet;

fn resolve_email_address(email: &str) -> Option<String> {
    let mut resolved = String::with_capacity(email.len());

    let mut at_pos: Option<usize> = None;
    let mut found_plus = false;
    for (i, c) in email.char_indices() {
        match c {
            '@' => {
                at_pos = Some(i);
                break;
            }
            '.' => (),
            '+' => {
                found_plus = true;
            }
            _ if found_plus => (),
            c => {
                resolved.push(c);
            }
        }
    }
    let at_pos = at_pos?;
    resolved.push_str(&email[at_pos..]);

    Some(resolved)
}

pub fn num_unique_emails(emails: Vec<String>) -> i32 {
    emails
        .into_iter()
        .filter_map(|e| resolve_email_address(&e))
        .collect::<HashSet<String>>()
        .len() as i32
}

fn main() {
    println!("Hello, world!");
}
