pub fn longest_common_subsequence<'a>(a: &'a [i64], b: &[i64]) -> Option<&'a [i64]> {
    let mut longest_len: usize = 0;
    let mut longest_start: Option<usize> = None;

    // we start by look at a's indices
    for r_idx in 0..a.len() {
        // state for the current streak
        let mut current_start: Option<usize> = None;
        let mut current_len: usize = 0;

        for candidate in b.iter() {
            // our position in a is offset by the streak length, so that we move into a as we find
            // commons in b.
            let r_pos = r_idx + current_len;
            // if we go past the end of a we can stop trying to extend this streak.
            let reference = match a.get(r_pos) {
                Some(c) => c,
                None => break,
            };
            // if we have a match we want to mark the start of it, if it's the first one, and
            // increment the streak length.
            // if we _don't_ have a match, then we want to update the longest streak, and start
            // counting again, there could be a longer streak further down the line!
            if reference == candidate {
                current_start.get_or_insert(r_pos);
                current_len += 1;
            } else {
                if current_len > longest_len {
                    longest_len = current_len;
                    longest_start = current_start;
                }
                current_len = 0;
                current_start = None;
            }
        }
        // We may have ended a long streak without finding a non-matching char, we need to make
        // sure to update the longest just in case.
        if current_len > longest_len {
            longest_len = current_len;
            longest_start = current_start;
        }
    }

    longest_start.and_then(|s| a.get(s..(s + longest_len)))
}

fn main() {
    assert_eq!(
        longest_common_subsequence(&[9, 1, 2, 3, 4, 5, 6], &[0, 0, 0, 1, 2, 3, 0]),
        Some(&[1, 2, 3][..])
    );

    assert_eq!(
        longest_common_subsequence(&[1, 2], &[1, 2]),
        Some(&[1, 2][..])
    );

    assert_eq!(
        longest_common_subsequence(&[2, 4, 5, 6, 8, 9], &[1, 1, 4, 5, 6, 6, 8, 0],),
        Some(&[4, 5, 6][..])
    );

    assert_eq!(
        longest_common_subsequence(&[1, 1, 4, 5, 6, 6, 8, 0], &[2, 4, 5, 6, 8, 9, 0, 10],),
        Some(&[4, 5, 6][..])
    );

    assert_eq!(
        longest_common_subsequence(&[1, 1, 2, 6, 0, 1, 2], &[1, 2, 6, 7, 1, 2],),
        Some(&[1, 2, 6][..])
    );
}
