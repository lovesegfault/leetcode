struct Solution;

impl Solution {
    pub fn intersect(mut nums1: Vec<i32>, mut nums2: Vec<i32>) -> Vec<i32> {
        use std::collections::hash_map::{Entry, HashMap, OccupiedEntry};

        if nums1.len() > nums2.len() {
            std::mem::swap(&mut nums1, &mut nums2);
        }

        let mut counts: HashMap<i32, usize> = nums2.iter().fold(HashMap::new(), |mut map, n| {
            *map.entry(*n).or_default() += 1;
            map
        });

        let mut out = Vec::with_capacity(nums1.len());

        for n in nums1 {
            match counts.entry(n) {
                Entry::Occupied(mut entry) => {
                    let count = entry.get_mut();
                    if *count > 0 {
                        *count -= 1;
                        out.push(n)
                    }
                }
                _ => continue,
            }
        }

        out
    }
}

fn main() {
    println!("Hello, world!");
}
