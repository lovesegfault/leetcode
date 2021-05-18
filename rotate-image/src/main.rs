struct Solution;

#[inline]
fn print_mat(mat: &Vec<Vec<i32>>) {
    mat.iter().for_each(|row| {
        print!("[ ");
        row.iter().for_each(|n| print!("{:02} ", n));
        println!("]");
    });
    println!("");
}

impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        // it's a square, nxn matrix
        let n = matrix.len() - 1;

        // we rotate it by layering, like an onion
        for layer in 0..(matrix.len() / 2) {
            // we'll need to do n-1 4-swaps for every layer
            for i in 0..(n - (2 * layer)) {
                // four our pivot point a, we pick 3 swap points: b, c, d where each is a's
                // position applied to the next "face" of this layer
                // (row, col)
                let (ar, ac) = (layer, layer + i);

                let (br, bc) = (layer + i, n - layer);

                let (cr, cc) = (n - layer, n - layer - i);

                let (dr, dc) = (n - layer - i, layer);

                let mut save = matrix[ar][ac];
                std::mem::swap(&mut matrix[br][bc], &mut save);
                std::mem::swap(&mut matrix[cr][cc], &mut save);
                std::mem::swap(&mut matrix[dr][dc], &mut save);
                std::mem::swap(&mut matrix[ar][ac], &mut save);
            }
        }
    }
}

fn main() {
    #[rustfmt::skip]
    let mut mat = vec![
        vec![1, 2, 3],
        vec![4, 5, 6],
        vec![7, 8, 9]
    ];
    Solution::rotate(&mut mat);
    print_mat(&mat);

    #[rustfmt::skip]
    let mut mat = vec![
        vec![5,  1,  9,  11],
        vec![2,  4,  8,  10],
        vec![13, 3,  6,  7],
        vec![15, 14, 12, 16],
    ];
    Solution::rotate(&mut mat);
    print_mat(&mat);
}
