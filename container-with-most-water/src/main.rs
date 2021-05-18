pub fn max_area(height: Vec<i32>) -> i32 {
    use std::cmp::Ordering;
    let mut best: usize = 0;
    let mut left: usize = 0;
    let mut right: usize = height.len() - 1;

    while left < right {
        // length of the base
        let base = right - left;
        // height of left and right wall
        let left_height = height[left] as usize;
        let right_height = height[right] as usize;

        match left_height.cmp(&right_height) {
            Ordering::Greater => {
                // update the best area
                best = best.max(base * right_height);
                // move the right wall
                right -= 1;
            }
            _ => {
                // update the best area
                best = best.max(base * left_height);
                left += 1;
            }
        }
    }
    best as i32
}

fn main() {
    assert_eq!(max_area(vec![1, 1]), 1);
    assert_eq!(max_area(vec![4, 3, 2, 1, 4]), 16);
    assert_eq!(max_area(vec![1, 2, 1]), 2);
}
