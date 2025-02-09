use wfc_sudoku::{cell, sudoku::{grid::SudokuGrid, number::SudokuNumber, region::RegionType}};

fn main() {
    let mut grid = SudokuGrid::default();

    grid.set_cell(1, 0, cell!(1));
    grid.set_cell(1, 1, cell!(5));
    grid.set_cell(1, 2, cell!(2));
    grid.set_cell(1, 5, cell!(5));
    grid.set_cell(1, 7, cell!(9));
    grid.set_cell(1, 8, cell!(8));

    grid.set_cell(4, 7, cell!(8));
    grid.set_cell(4, 8, cell!(8));

    grid.set_cell(5, 8, cell!(8));

    grid.set_cell(8, 2, cell!(2));

    println!("{}", grid);

    for idx in 0..9 {
        if let Err(e) = grid.check_row(idx) {
            println!("Err: Repetition on line {}. Repeating numbers:", idx + 1);
            for num in e {
                println!("{}", num);
            }
        }
    }

    for idx in 0..9 {
        if let Err(e) = grid.check_col(idx) {
            println!("Err: Repetition on column {}. Repeating numbers:", idx + 1);
            for num in e {
                println!("{}", num);
            }
        }
    }

    for i in 0..9 {
        grid.get_region(i, RegionType::Box)
            .iter()
            .for_each(|cell| print!("{}", cell));

        println!();
    }
}
