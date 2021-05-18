struct Solution;

use std::collections::HashSet;
use std::ops::Range;
impl Solution {
    fn validate_row(board: &Vec<Vec<char>>, idx: usize) -> bool {
        let row = &board[idx];
        let mut seen: HashSet<char> = HashSet::with_capacity(9);

        !row.iter()
            .filter(|&&cell| cell != '.')
            .map(|cell| seen.insert(*cell))
            .any(|dupe| dupe == false)
    }

    fn validate_col(board: &Vec<Vec<char>>, idx: usize) -> bool {
        let mut seen: HashSet<char> = HashSet::with_capacity(9);

        for i in 0..9 {
            let cell = board[i][idx];

            if cell == '.' {
                continue;
            }

            if !seen.insert(cell) {
                return false;
            }
        }

        true
    }

    fn validate_box(board: &Vec<Vec<char>>, idx: usize) -> bool {
        const BOX_RANGES: [(Range<usize>, Range<usize>); 9] = [
            (0..3, 0..3),
            (3..6, 0..3),
            (6..9, 0..3),
            (0..3, 3..6),
            (3..6, 3..6),
            (6..9, 3..6),
            (0..3, 6..9),
            (3..6, 6..9),
            (6..9, 6..9),
        ];

        let (row_range, col_range) = BOX_RANGES[idx].clone();

        let mut seen: HashSet<char> = HashSet::with_capacity(9);
        for x in row_range {
            for y in col_range.clone() {
                let cell = board[x][y];
                if cell == '.' {
                    continue;
                }

                if !seen.insert(cell) {
                    return false;
                }
            }
        }

        true
    }

    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        !(0..9)
            .into_iter()
            .map(|i| {
                Self::validate_row(&board, i)
                    && Self::validate_col(&board, i)
                    && Self::validate_box(&board, i)
            })
            .any(|valid| valid == false)
    }
}

fn main() {
    let board = vec![
        vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
        vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
        vec!['.', '9', '6', '.', '.', '.', '.', '6', '.'],
        vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
        vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
        vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
        vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
        vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
        vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
    ];

    dbg!(Solution::is_valid_sudoku(board));
}
