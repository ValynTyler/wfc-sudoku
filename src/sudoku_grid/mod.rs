use std::fmt::Display;

use crate::sudoku_cell::SudokuCell;

pub struct SudokuGrid(pub [[SudokuCell; 9]; 9]);

impl Default for SudokuGrid {
    fn default() -> Self {
        Self([[SudokuCell::default(); 9]; 9])
    }
}

impl Display for SudokuGrid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (idx, row) in self.0.iter().enumerate() {
            for (index, cell) in row.iter().enumerate() {
                write!(f, "{}", cell)?;
                if (index + 1) % 3 == 0 && (index + 1) < 9 {
                    write!(f, " | ")?;
                }
            }
            writeln!(f)?;
            if (idx + 1) % 3 == 0 && (idx + 1) < 9 {
                for _ in 0..15 {
                    write!(f, "-")?;
                }
                writeln!(f)?;
            }
        }

        Ok(())
    }
}
