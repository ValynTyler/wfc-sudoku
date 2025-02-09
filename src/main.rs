use wfc_sudoku::{cell, sudoku_grid::SudokuGrid, sudoku_number::SudokuNumber};

fn main() {
    let mut grid = SudokuGrid::default();

    grid.set_cell(1, 0, cell!(1));
    grid.set_cell(1, 1, cell!(5));
    grid.set_cell(1, 2, cell!(1));
    grid.set_cell(1, 5, cell!(5));
    grid.set_cell(1, 7, cell!(9));
    grid.set_cell(1, 8, cell!(9));

    println!("{}", grid);

    if let Err(e) = grid.check_row(1) {
        println!("Err: repeating numbers:");
        for num in e {
            println!("{}", num);
        }
    }
}
