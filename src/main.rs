use wfc_sudoku::sudoku_grid::SudokuGrid;

fn main() {
    let mut grid = SudokuGrid::default();

    grid.set_cell((2, 2), 1).unwrap();
    grid.set_cell((5, 2), 2).unwrap();
    grid.set_cell((8, 2), 3).unwrap();

    grid.set_cell((2, 5), 4).unwrap();
    grid.set_cell((5, 5), 5).unwrap();
    grid.set_cell((8, 5), 6).unwrap();

    grid.set_cell((2, 8), 7).unwrap();
    grid.set_cell((5, 8), 8).unwrap();
    grid.set_cell((8, 8), 9).unwrap();

    println!("{}", grid);
}
