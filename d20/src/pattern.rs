use crate::tile::Tile;
use simple_error::{bail, SimpleError, SimpleResult};
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::ops::RangeInclusive;

pub struct Pattern {
    data: HashMap<(i16, i16), Tile>,
    tile_ids: HashSet<u16>,
}

impl Pattern {
    pub fn new() -> Self {
        Pattern {
            data: HashMap::new(),
            tile_ids: HashSet::new(),
        }
    }
    pub fn try_insert(&mut self, tile: &Tile, x: i16, y: i16) -> SimpleResult<()> {
        if self.data.contains_key(&(y, x)) {
            bail!("cell already occupied");
        } else if self.tile_ids.contains(&tile.get_number()) {
            bail!("tile id already in pattern");
        } else {
            let mut cells_found = 0;
            let mut cells_matching = 0;

            if let Some(other_tile) = self.data.get(&(y - 1, x)) {
                cells_found += 1;

                if tile.first_row() == other_tile.last_row() {
                    cells_matching += 1;
                }
            }

            if let Some(other_tile) = self.data.get(&(y + 1, x)) {
                cells_found += 1;

                if tile.last_row() == other_tile.first_row() {
                    cells_matching += 1;
                }
            }

            if let Some(other_tile) = self.data.get(&(y, x - 1)) {
                cells_found += 1;
                if tile.first_column() == other_tile.last_column() {
                    cells_matching += 1;
                }
            }

            if let Some(other_tile) = self.data.get(&(y, x + 1)) {
                cells_found += 1;
                if tile.last_column() == other_tile.first_column() {
                    cells_matching += 1;
                }
            }

            if cells_found == 0 && !(self.data.is_empty()) {
                bail!("no neigbour cells in non-empty pattern");
            }

            if cells_found != cells_matching {
                bail!("not all cells match");
            }

            self.data.insert((y, x), tile.clone());
            self.tile_ids.insert(tile.get_number());
        }

        Ok(())
    }

    fn range_x(&self) -> SimpleResult<RangeInclusive<i16>> {
        if self.data.is_empty() {
            bail!("empty data");
        } else {
            let min = self.data.keys().map(|(_y, x)| x).min().unwrap();
            let max = self.data.keys().map(|(_y, x)| x).max().unwrap();
            Ok(*min..=*max)
        }
    }

    fn range_y(&self) -> SimpleResult<RangeInclusive<i16>> {
        if self.data.is_empty() {
            bail!("empty data");
        } else {
            let min = self.data.keys().map(|(y, _x)| y).min().unwrap();
            let max = self.data.keys().map(|(y, _x)| y).max().unwrap();
            Ok(*min..=*max)
        }
    }
}

impl fmt::Display for Pattern {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.data.is_empty() {
            return Ok(());
        }

        for y in self.range_y().unwrap() {
            for x in self.range_x().unwrap() {
                let idx = (y, x);
                if let Some(tile) = self.data.get(&idx) {
                    write!(f, "{}\t", tile.get_number())?;
                } else {
                    write!(f, "    \t")?;
                }
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test]
    fn test_empty() {
        let p = Pattern::new();
        assert_eq!(format!("{}", p), "");
    }

    #[test]
    fn test_single_tile() {
        let mut p = Pattern::new();
        // insert into empty pattern is successful
        p.try_insert(&(Tile::new(1234)), 0, 0).unwrap();
        assert_eq!(format!("{}", p), "1234\t\n");

        // insert again into the same cell fails
        assert!(p.try_insert(&(Tile::new(4321)), 0, 0).is_err());

        // insert same number into different far away cell fails
        assert!(p.try_insert(&(Tile::new(1234)), 10, 10).is_err());
    }

    #[test_case(
        "Tile 1:\n\
        #.#\n\
        ..#\n\
        ..#",
        1,
        0,
        "Tile 2:\n\
        #.#\n\
        #..\n\
        #.#",
        "1\t2\t\n"; 
        "right"
    )]
    #[test_case(
        "Tile 1:\n\
        #.#\n\
        ..#\n\
        ..#",
        -1,
        0,
        "Tile 2:\n\
        #.#\n\
        #..\n\
        #..",
        "2\t1\t\n";
        "left"
    )]
    #[test_case(
        "Tile 1:\n\
        #.#\n\
        ..#\n\
        ..#",
        0,
        1,
        "Tile 2:\n\
        ..#\n\
        #..\n\
        #.#",
        "1\t\n2\t\n"; 
        "down"
    )]
    #[test_case(
        "Tile 1:\n\
        #.#\n\
        ..#\n\
        ..#",
        0,
        -1,
        "Tile 2:\n\
        #..\n\
        #..\n\
        #.#", 
        "2\t\n1\t\n"; 
        "up"
    )]
    fn test_try_insert(input_0_0: &str, x: i16, y: i16, new_tile: &str, expected: &str) {
        // intial setup
        let mut p = Pattern::new();
        let input_tile: Tile = input_0_0.parse().unwrap();
        p.try_insert(&input_tile, 0, 0).unwrap();

        // new tile
        let new_tile: Tile = new_tile.parse().unwrap();
        p.try_insert(&new_tile, x, y).unwrap();

        assert_eq!(format!("{}", p), expected);
    }

    #[test_case(
        "Tile 1:\n\
        #.#\n\
        ..#\n\
        ..#",
        1,
        0,
        "Tile 2:\n\
        #.#\n\
        ...\n\
        #.#"; 
        "new tile on the right, pattern mismatch"
    )]
    #[test_case(
        "Tile 1:\n\
        #.#\n\
        ..#\n\
        ..#",
        2,
        0,
        "Tile 2:\n\
        #.#\n\
        #..\n\
        #.#"; 
        "good match, but not directly next to each other"
    )]
    fn test_try_insert_fail(input_0_0: &str, x: i16, y: i16, new_tile: &str) {
        // intial setup
        let mut p = Pattern::new();
        let input_tile: Tile = input_0_0.parse().unwrap();
        p.try_insert(&input_tile, 0, 0).unwrap();

        // new tile
        let new_tile: Tile = new_tile.parse().unwrap();
        assert!(p.try_insert(&new_tile, x, y).is_err());
    }
}
