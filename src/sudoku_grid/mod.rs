use crate::sudoku_cell::SudokuCell;

pub struct SudokuGrid(pub [[SudokuCell; 9]; 9]);

impl Default for SudokuGrid {
    fn default() -> Self {
        Self([[SudokuCell::default(); 9]; 9])
    }
}
