use std::fmt::Display;

use crate::sudoku::cell::*;
use crate::sudoku::number::*;
use crate::sudoku::region::*;

pub struct SudokuGrid(pub [[SudokuCell; 9]; 9]);

impl SudokuGrid {
    pub fn get_cell(&self, row: usize, col: usize) -> SudokuCell {
        self.0[row][col].clone()
    }

    pub fn get_region(&self, index: usize, region_type: RegionType) -> Region {
        match region_type {
            RegionType::Box => {
                let row_offset = (index / 3) * 3;
                let col_offset = (index % 3) * 3;

                let box_shape: Vec<_> = vec![
                    (0, 0), (0, 1), (0, 2),
                    (1, 0), (1, 1), (1, 2),
                    (2, 0), (2, 1), (2, 2),
                ];

                Region(box_shape
                    .iter()
                    .map(|pos| self.get_cell(pos.0 + row_offset, pos.1 + col_offset))
                    .collect::<Vec<_>>()
                )
            }
            RegionType::Row => Region(self.0[index].to_vec()),
            RegionType::Column => Region(self.0
                .iter()
                .map(|row| row[index])
                .collect::<Vec<_>>()
            )
        }
    }

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

    pub fn check(&self) {

        let jeff = self.get_region(0, RegionType::Row).check();
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
