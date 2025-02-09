use std::fmt::Display;

#[derive(Clone, Copy)]
pub enum SudokuNumber {
    _1,
    _2,
    _3,
    _4,
    _5,
    _6,
    _7,
    _8,
    _9,
}

impl Display for SudokuNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SudokuNumber::_1 => write!(f, "{}", 1),
            SudokuNumber::_2 => write!(f, "{}", 2),
            SudokuNumber::_3 => write!(f, "{}", 3),
            SudokuNumber::_4 => write!(f, "{}", 4),
            SudokuNumber::_5 => write!(f, "{}", 5),
            SudokuNumber::_6 => write!(f, "{}", 6),
            SudokuNumber::_7 => write!(f, "{}", 7),
            SudokuNumber::_8 => write!(f, "{}", 8),
            SudokuNumber::_9 => write!(f, "{}", 9),
        }
    }
}
