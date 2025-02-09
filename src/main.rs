use wfc_sudoku::{cell, sudoku_grid::SudokuGrid, sudoku_number::SudokuNumber};

fn main() {
    let mut grid = SudokuGrid::default();

    for i in 1..10 {
        for j in 1..10 {
            let num = 9 - (i + j) % 9;
            grid.set_cell(j, i, cell!(num));
        }
    }

    println!("{}", grid);
}
