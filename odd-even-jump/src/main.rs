pub fn odd_even_jump_no_memo(arr: Vec<i32>) -> i32 {
    fn inner(arr: &[i32], jump_is_odd: bool) -> bool {
        if arr.len() == 1 {
            return true;
        }

        let mut remainder_iter = arr.iter().enumerate();
        let reference = remainder_iter.next().map(|(_, v)| v).unwrap(); // we _know_ that this array has at least two elements.

        let next_pos = if jump_is_odd {
            remainder_iter
                .filter(|&(_, v)| reference <= v)
                .min_by_key(|(_, v)| *v)
                .map(|(i, _)| i)
        } else {
            remainder_iter
                .filter(|&(_, v)| reference >= v)
                .min_by_key(|(_, v)| *v * -1)
                .map(|(i, _)| i)
        };

        let reaches_end = next_pos
            .map(|pos| inner(&arr[pos..], !jump_is_odd))
            .unwrap_or(false);

        reaches_end
    }

    (0..arr.len()).filter(|i| inner(&arr[*i..], true)).count() as i32
}

pub fn odd_even_jumps_memo(arr: Vec<i32>) -> i32 {
    type Memo = std::collections::HashMap<(usize, bool), bool>;

    fn inner(memo: &mut Memo, arr: &[i32], start: usize, jump_is_odd: bool) -> bool {
        if start == arr.len() - 1 {
            return true;
        }

        if let Some(previous) = memo.get(&(start, jump_is_odd)) {
            return *previous;
        }

        let mut remainder_iter = arr.iter().enumerate().skip(start);
        let reference = remainder_iter
            .next()
            .map(|(_, v)| v)
            .expect("should be unreacheable");

        let next_pos = if jump_is_odd {
            remainder_iter
                .filter(|(_, v)| reference <= v)
                .min_by_key(|(_, v)| *v)
                .map(|(i, _)| i)
        } else {
            remainder_iter
                .filter(|&(_, v)| reference >= v)
                .min_by_key(|(_, v)| *v * -1)
                .map(|(i, _)| i)
        };

        let reaches_end = next_pos
            .map(|pos| inner(memo, &arr, pos, !jump_is_odd))
            .unwrap_or(false);

        memo.insert((start, jump_is_odd), reaches_end);
        reaches_end
    }

    let mut memo = Memo::with_capacity(arr.len());
    (0..arr.len())
        .filter(|i| inner(&mut memo, &arr, *i, true))
        .count() as i32
}

pub fn odd_even_jumps(arr: Vec<i32>) -> i32 {
    use std::collections::BTreeMap;
    if arr.is_empty() {
        return 0;
    }

    // K = values in arr
    // V = (odd jump works; even jump works)
    let mut seen: BTreeMap<i32, (bool, bool)> = BTreeMap::new();

    // let last position is always successful
    seen.insert(*arr.last().unwrap(), (true, true));
    let mut count = 1;

    // start from the back
    for &value in arr.iter().rev().skip(1) {
        let odd_jump_works = seen
            .range(value..)
            .next()
            .map(|(_, &(_, successor_even_works))| successor_even_works)
            .unwrap_or(false);
        let even_jump_works = seen
            .range(..=value)
            .last()
            .map(|(_, &(predecessor_odd_works, _))| predecessor_odd_works)
            .unwrap_or(false);

        if odd_jump_works {
            count += 1;
        }

        seen.insert(value, (odd_jump_works, even_jump_works));
    }
    count
}

fn main() {
    assert_eq!(odd_even_jumps(vec![10, 13, 12, 14, 15]), 2);
    assert_eq!(odd_even_jumps(vec![2, 3, 1, 1, 4]), 3);
    assert_eq!(odd_even_jumps(vec![5, 1, 3, 4, 2]), 3);
    assert_eq!(odd_even_jumps(vec![81, 54, 96, 60, 58]), 2);
}
