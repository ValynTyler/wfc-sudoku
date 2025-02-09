use std::fmt::Display;

use crate::{sudoku_cell::SudokuCell, sudoku_number::SudokuNumber};

pub struct SudokuGrid(pub [[SudokuCell; 9]; 9]);

impl SudokuGrid {
    pub fn set_cell(&mut self, pos: (usize, usize), value: u8) -> Result<(), ()> {
        self.0[pos.1 - 1][pos.0 - 1] = SudokuCell(Some(
            SudokuNumber::try_from(value)?
        ));

        Ok(())
    }
}

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
