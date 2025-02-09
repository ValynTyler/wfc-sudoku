use std::fmt::Display;

use crate::sudoku_number::SudokuNumber;

#[derive(Clone, Copy)]
pub struct SudokuCell(pub Option<SudokuNumber>);

#[macro_export]
macro_rules! cell {
    () => { None };
    ($x:expr) => {
        match $x {
            1 => Some(SudokuNumber::_1),
            2 => Some(SudokuNumber::_2),
            3 => Some(SudokuNumber::_3),
            4 => Some(SudokuNumber::_4),
            5 => Some(SudokuNumber::_5),
            6 => Some(SudokuNumber::_6),
            7 => Some(SudokuNumber::_7),
            8 => Some(SudokuNumber::_8),
            9 => Some(SudokuNumber::_9),
            _ => None,
        };
    }
}

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
