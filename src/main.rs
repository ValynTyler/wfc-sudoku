use wfc_sudoku::{cell, sudoku_grid::SudokuGrid, sudoku_number::SudokuNumber};

fn main() {
    let mut grid = SudokuGrid::default();

    grid.set_cell(2, 2, cell!(1));
    grid.set_cell(5, 2, cell!(2));
    grid.set_cell(8, 2, cell!(3));

    grid.set_cell(2, 5, cell!(4));
    grid.set_cell(5, 5, cell!(5));
    grid.set_cell(8, 5, cell!(6));

    grid.set_cell(2, 8, cell!(7));
    grid.set_cell(5, 8, cell!(8));
    grid.set_cell(8, 8, cell!(9));

    println!("{}", grid);
}
