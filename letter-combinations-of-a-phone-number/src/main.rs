use phf::{phf_map, Map};

pub fn phone_to_words(phone_number: &str) -> Vec<String> {
    if phone_number.is_empty() {
        return Vec::new();
    }
    // we use a perfect hash function to generate a compile-time efficient map
    static NUM_MAP: Map<char, &'static [char]> = phf_map! {
        '2' => &['a', 'b', 'c'],
        '3' => &['d', 'e', 'f'],
        '4' => &['g', 'h', 'i'],
        '5' => &['j', 'k', 'f'],
        '6' => &['m', 'n', 'o'],
        '7' => &['p', 'q', 'r', 's'],
        '8' => &['t', 'u', 'v'],
        '9' => &['w', 'x', 'y', 'z'],
    };

    // for every digit in our number
    phone_number
        .chars()
        .fold(vec![String::new()], |acc, digit| {
            acc.into_iter() // for every string in our accumulator vec
                .map(|old| {
                    NUM_MAP[&digit]
                        .iter() // for every possible new digit
                        .map(|new| format!("{}{}", old, new)) // construct a new string of the old string with the new digit
                        .collect::<Vec<String>>() // collect to a vec of the current combinations
                })
                .flatten() // Iter of Vec<T> -> Iter of T
                .collect() // return the flattened vec
        })
}

pub fn phone_to_words2(phone_number: &str) -> Vec<String> {
    if phone_number.is_empty() {
        return Vec::new();
    }

    // we use a const slice as a poor man's hashmap from digit to possible characters
    const NUM_MAP: &[&str] = &[
        "unreacheable", // 0
        "unreacheable", // 1
        "abc",          // 2
        "def",          // 3
        "ghi",          // 4
        "jkf",          // 5
        "mno",          // 6
        "pqrs",         // 7
        "tuv",          // 8
        "wxyz",         // 9
    ];

    let mut result: Vec<String> = Vec::new();

    // for every digit in our phone number
    for digit in phone_number.chars() {
        let mut tmp = Vec::new();
        let digit = digit.to_digit(10).unwrap() as usize; // convert to base 10 numeric type
        let letters = NUM_MAP[digit]; // possible letters for this digit

        // for each possible letter
        for letter in letters.chars() {
            // if this is our first filling of result, just push them indiscriminately
            if result.is_empty() {
                tmp.push(letter.to_string());
            } else {
                // otherwise, we want to append this letter to every string previously held in
                // result.
                for old in result.iter() {
                    tmp.push(old.to_owned() + &letter.to_string())
                }
            }
        }
        // move tmp, which now holds the updated list of possible words, into result.
        result = tmp;
    }

    result
}

pub fn main() {
    phone_to_words("234")
        .iter()
        .for_each(|combination| println!("{}", combination));
    println!("------");
    phone_to_words2("234")
        .iter()
        .for_each(|combination| println!("{}", combination));
}
