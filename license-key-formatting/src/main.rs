pub fn license_key_formatting(s: String, k: i32) -> String {
    let mut out = String::with_capacity(s.len() + (s.len() / k as usize));
    let sb = s.as_bytes();

    let mut count = 0;
    for &c in sb.iter().rev() {
        if c == b'-' {
            continue;
        }
        out.push((c as char).to_ascii_uppercase());

        count += 1;
        if count % k == 0 {
            out.push('-')
        }
    }

    if out.chars().last() == Some('-') {
        out.pop();
    }

    unsafe { out.as_bytes_mut().reverse() };
    out
}

fn shim(s: &str, k: i32) -> String {
    license_key_formatting(s.to_string(), k)
}

fn main() {
    dbg!(shim("5F3Z-2e-9-w", 4));
    dbg!(shim("2-5g-3-J", 2));
    dbg!(shim("--a-a-a-a--", 2));
}
