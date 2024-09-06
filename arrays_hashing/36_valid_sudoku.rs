/// Complexity: O(1) => Board size stays the same and we always iterate the entire thing.
///
/// We split into 3 rules, which are the rows, columns, and squares, and access each of them for
/// every element on the board, returning false if the number is already present for any of the
/// rules.
use std::collections::HashMap;

impl Solution {
    // TODO: This can be improved using bit manipulation, will do later
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        // Have data structures for the three rules
        // Validate rows and columns using idx
        // Validate squares by using a tuple pair
        let mut rows = [[false; 10]; 10];
        let mut cols = [[false; 10]; 10];
        let mut sqrs: HashMap<(usize, usize), [bool; 10]> = HashMap::new();

        for (i, row) in board.into_iter().enumerate() {
            for (j, item) in row.into_iter().enumerate() {
                match item {
                    '.' => continue,
                    c => {
                        // Convert to digit
                        let c = (c as u8 - b'0') as usize;
                        // Get square entry
                        let ent = sqrs.entry((i / 3, j / 3)).or_insert([false; 10]);
                        // Check if exists
                        if rows[i][c] || cols[j][c] || ent[c] {
                            return false;
                        }
                        // Set digits
                        rows[i][c] = true;
                        cols[j][c] = true;
                        ent[c] = true;
                    }
                }
            }
        }
        true
    }
}
