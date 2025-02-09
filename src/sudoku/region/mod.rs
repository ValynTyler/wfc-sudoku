use super::{cell::SudokuCell, number::SudokuNumber};

pub enum RegionType {
    Box,
    Row,
    Column,
}

pub struct Region(pub Vec<SudokuCell>);

impl Region {
    pub fn check(self) -> Result<(), Vec<SudokuNumber>> {
        let mut freq = [ 0; 10 ];
        let mut errs = vec![];

        for cell in self.0 {
            if let Some(n) = cell.0 {
                freq[usize::from(n)] += 1;
            }
        }

        for (idx, num) in freq.into_iter().enumerate() {
            if num > 1 {
                errs.push(SudokuNumber::try_from(idx).unwrap());
            }
        }

        match errs.len() {
            0 => Ok(()),
            _ => Err(errs),
        }
    }
}
