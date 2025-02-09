use crate::sudoku_number::SudokuNumber;

#[derive(Clone, Copy)]
pub struct SudokuCell(pub Option<SudokuNumber>);

impl Default for SudokuCell {
    fn default() -> Self {
        Self(None)
    }
}
