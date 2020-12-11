use simple_error::SimpleError;
use std::fmt;

#[derive(Debug, Clone)]
pub struct Seats {
    // TODO: use 2d vector
    data: Vec<Vec<Option<bool>>>,
    cols: usize,
    rows: usize,
}

impl fmt::Display for Seats {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.data.iter() {
            writeln!(
                f,
                "{}",
                line.iter()
                    .map(|s| match s {
                        None => ".",
                        Some(false) => "L",
                        Some(true) => "#",
                    })
                    .collect::<String>()
            )?
        }
        Ok(())
    }
}

impl Seats {
    pub fn new(rows: usize, cols: usize) -> Self {
        Seats {
            data: vec![vec![None; cols]; rows],
            cols,
            rows,
        }
    }
    pub fn get(&self, row: i64, col: i64) -> Result<Option<bool>, SimpleError> {
        if row < 0 || col < 0 || row >= (self.rows as i64) || col >= (self.cols as i64) {
            Err(SimpleError::new("invalid row/col"))
        } else {
            Ok(self.data[row as usize][col as usize])
        }
    }

    pub fn set(&mut self, row: i64, col: i64, value: Option<bool>) -> Result<(), SimpleError> {
        if row >= 0 && col >= 0 || row < (self.rows as i64) || col < (self.cols as i64) {
            self.data[row as usize][col as usize] = value;
            Ok(())
        } else {
            Err(SimpleError::new("invalid row/col"))
        }
    }

    pub fn count(&self, which: Option<bool>) -> usize {
        self.data
            .iter()
            .map(|row| row.iter().filter(|v| **v == which).count())
            .sum()
    }

    pub fn cols(&self) -> usize {
        self.cols
    }

    pub fn rows(&self) -> usize {
        self.rows
    }

    pub fn from(lines: impl Iterator<Item = String>) -> Self {
        let data: Vec<Vec<Option<bool>>> = lines
            .map(|l| {
                l.chars()
                    .map(|c| match c {
                        'L' => Some(false),
                        '#' => Some(true),
                        _ => None,
                    })
                    .collect()
            })
            .collect();

        if data.len() > 0 {
            let cols = data[0].len();
            let rows = data.len();
            Seats { data, cols, rows }
        } else {
            Seats::new(0, 0)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn get_set() {
        let mut seats = Seats::new(3, 2);

        assert_eq!(seats.rows(), 3);
        assert_eq!(seats.cols(), 2);

        assert_eq!(seats.set(2, 1, Some(true)), Ok(()));
        assert_eq!(seats.get(2, 1), Ok(Some(true)));

        assert!(seats.get(5, 5).is_err());
        assert!(seats.get(3, 2).is_err());
    }
}
