pub fn total_fruit(trees: Vec<i32>) -> i32 {
    let mut last: Option<i32> = None;
    let mut last_count: usize = 0;
    let mut second_to_last: Option<i32> = None;

    let mut current: usize = 0;
    let mut result: usize = 0;

    for fruit in trees {
        let fruit = Some(fruit);
        if fruit == last {
            // if this is the same as the last fruit we've seen, add one to the current streak
            // count for this fruit, and to the total for this general streak.
            current += 1;
            last_count += 1;
        } else if fruit == second_to_last {
            // if this fruit is the same as the second to last fruit, we increment the general
            // count, but we reset the last count.
            // the previous last will now be the second to last, and the current fruit is the last
            // one we've seen.
            current += 1;
            last_count = 1;
            second_to_last = last;
            last = fruit;
        } else {
            // if this fruit is neither of the two fruits we have, then we update the current count
            // to be the amount of the last fruit, plus the corrent one we're looking at.
            // the previous last will not be the second to last, and the current fruit is the new
            // last.
            current = last_count + 1;
            last_count = 1;
            second_to_last = last;
            last = fruit;
        }
        result = result.max(current);
    }

    result as i32
}

fn main() {
    assert_eq!(total_fruit(vec![1, 2, 1]), 3);
    assert_eq!(total_fruit(vec![0, 1, 2, 2]), 3);
    assert_eq!(total_fruit(vec![1, 2, 3, 2, 2]), 4);
    assert_eq!(total_fruit(vec![0, 1, 6, 6, 4, 4, 6]), 5);
    assert_eq!(total_fruit(vec![1, 0, 1, 4, 1, 4, 1, 2, 3]), 5);
    assert_eq!(total_fruit(vec![3, 3, 3, 1, 2, 1, 1, 2, 3, 3, 4]), 5);
}
