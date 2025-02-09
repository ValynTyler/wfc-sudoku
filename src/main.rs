use wfc_sudoku::{cell, sudoku_grid::SudokuGrid, sudoku_number::SudokuNumber};

fn main() {
    let mut grid = SudokuGrid::default();

    grid.set_cell((2, 2), cell!(1)).unwrap();
    grid.set_cell((5, 2), cell!(2)).unwrap();
    grid.set_cell((8, 2), cell!(3)).unwrap();

    grid.set_cell((2, 5), cell!(4)).unwrap();
    grid.set_cell((5, 5), cell!(5)).unwrap();
    grid.set_cell((8, 5), cell!(6)).unwrap();

    grid.set_cell((2, 8), cell!(7)).unwrap();
    grid.set_cell((5, 8), cell!(8)).unwrap();
    grid.set_cell((8, 8), cell!(9)).unwrap();

    println!("{}", grid);
}
