use std::fmt::Display;

use crate::sudoku_number::SudokuNumber;

#[derive(Clone, Copy)]
pub struct SudokuCell(pub Option<SudokuNumber>);

impl Default for SudokuCell {
    fn default() -> Self {
        Self(None)
    }
}

impl Display for SudokuCell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            Some(n) => write!(f, "{}", n),
            None => write!(f, " "),
        }
    }
}
