struct Solution;

impl Solution {
    pub fn first_uniq_char(s: String) -> i32 {
        use std::collections::{BTreeSet, HashMap};

        let mut seen: HashMap<char, usize> = HashMap::with_capacity(s.len());
        let mut positions: BTreeSet<usize> = BTreeSet::new();

        for (i, c) in s.char_indices() {
            match seen.insert(c, i) {
                None => {
                    // this is a new char, add it to the pos set
                    positions.insert(i);
                }
                Some(pos) => {
                    // this is a duplicate, remove it from the pos set
                    positions.remove(&pos);
                }
            }
        }

        positions.into_iter().next().map(|f| f as i32).unwrap_or(-1)
    }

    pub fn first_uniq_char2(s: String) -> i32 {
        let mut seen = [0; 26];
        s.chars().for_each(|c| seen[c as usize - 97] += 1);
        s.char_indices()
            .filter(|(_, c)| seen[*c as usize - 97] <= 1)
            .next()
            .map(|(i, _)| i as i32)
            .unwrap_or(-1)
    }
}

fn main() {
    println!("Hello, world!");
}
