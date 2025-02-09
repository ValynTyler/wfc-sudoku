use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
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

impl From::<SudokuNumber> for usize {
    fn from(value: SudokuNumber) -> Self {
        match value {
            SudokuNumber::_1 => 1,
            SudokuNumber::_2 => 2,
            SudokuNumber::_3 => 3,
            SudokuNumber::_4 => 4,
            SudokuNumber::_5 => 5,
            SudokuNumber::_6 => 6,
            SudokuNumber::_7 => 7,
            SudokuNumber::_8 => 8,
            SudokuNumber::_9 => 9,
        }
    }
}

impl TryFrom::<usize> for SudokuNumber {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::_1),
            2 => Ok(Self::_2),
            3 => Ok(Self::_3),
            4 => Ok(Self::_4),
            5 => Ok(Self::_5),
            6 => Ok(Self::_6),
            7 => Ok(Self::_7),
            8 => Ok(Self::_8),
            9 => Ok(Self::_9),
            _ => Err(()),
        }
    }
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
