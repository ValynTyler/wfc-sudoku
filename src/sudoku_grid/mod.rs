use std::fmt::Display;

use crate::{sudoku_cell::SudokuCell, sudoku_number::SudokuNumber};

pub struct SudokuGrid(pub [[SudokuCell; 9]; 9]);

impl SudokuGrid {
    pub fn set_cell(
        &mut self,
        row: usize,
        col: usize,
        value: Option<SudokuNumber>
    ) {
        let col = col % 9;
        let row = row % 9;

        self.0[row][col] = SudokuCell(value);
    }
}

impl Default for SudokuGrid {
    fn default() -> Self {
        Self([[SudokuCell::default(); 9]; 9])
    }
}

impl Display for SudokuGrid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fn grid_line() -> String {
            format!("{}+", "+-------".repeat(3))
        }

        fn number_line(row: &[SudokuCell]) -> String {
            format!("| {}|", row
                .iter()
                .enumerate()
                .map(|(index, cell)| {
                    match index == 3 || index == 6  {
                        true => format!("| {} ", cell),
                        false => format!("{} ", cell),
                    }
                })
                .collect::<String>()
            )
        }

        writeln!(f, "{}", grid_line())?;
        writeln!(f, "{}", number_line(&self.0[0]))?;
        writeln!(f, "{}", number_line(&self.0[1]))?;
        writeln!(f, "{}", number_line(&self.0[2]))?;
        writeln!(f, "{}", grid_line())?;
        writeln!(f, "{}", number_line(&self.0[3]))?;
        writeln!(f, "{}", number_line(&self.0[4]))?;
        writeln!(f, "{}", number_line(&self.0[5]))?;
        writeln!(f, "{}", grid_line())?;
        writeln!(f, "{}", number_line(&self.0[6]))?;
        writeln!(f, "{}", number_line(&self.0[7]))?;
        writeln!(f, "{}", number_line(&self.0[8]))?;
        write!(f, "{}", grid_line())?;

        Ok(())
    }
}
