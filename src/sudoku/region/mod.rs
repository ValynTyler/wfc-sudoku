use super::{cell::SudokuCell, number::SudokuNumber};

pub enum RegionType {
    Box,
    Row,
    Column,
}

pub struct Region(pub Vec<SudokuCell>);

impl Region {
    pub fn get_repetitions(&self) -> Vec<SudokuNumber> {
        let mut freq: [ usize; 10 ] = [ 0; 10 ];
        let mut reps = vec![];

        for cell in &self.0 {
            if let Some(n) = cell.0 {
                freq[usize::from(n)] += 1;
            }
        }

        for (idx, num) in freq.into_iter().enumerate() {
            if num > 1 {
                reps.push(SudokuNumber::try_from(idx).unwrap());
            }
        }

        reps
    }

    pub fn check(&self) -> Result<(), Vec<SudokuNumber>> {
        let reps = self.get_repetitions();
        match reps.len() {
            0 => Ok(()),
            _ => Err(reps),
        }
    }
}
