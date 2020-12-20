use crate::tile::Tile;
use simple_error::{bail, SimpleResult};
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::ops::Range;

#[derive(Debug, Clone, PartialEq)]
pub struct Pattern {
    data: HashMap<(u16, u16), Tile>,
    tile_ids: HashSet<u16>,
    size: u16,
}

impl Pattern {
    pub fn new(size: u16) -> Self {
        Pattern {
            data: HashMap::new(),
            tile_ids: HashSet::new(),
            size,
        }
    }
    pub fn try_insert(&mut self, tile: &Tile, x: u16, y: u16) -> SimpleResult<()> {
        if x >= self.size || y >= self.size {
            bail!("index out of bounds");
        } else if self.data.contains_key(&(y, x)) {
            bail!("cell already occupied");
        } else if self.tile_ids.contains(&tile.get_number()) {
            bail!("tile id already in pattern");
        } else {
            let mut cells_found = 0;
            let mut cells_matching = 0;

            if y > 0 {
                if let Some(other_tile) = self.data.get(&(y - 1, x)) {
                    cells_found += 1;

                    if tile.first_row() == other_tile.last_row() {
                        cells_matching += 1;
                    }
                }
            }

            if self.range_y().contains(&(y + 1)) {
                if let Some(other_tile) = self.data.get(&(y + 1, x)) {
                    cells_found += 1;

                    if tile.last_row() == other_tile.first_row() {
                        cells_matching += 1;
                    }
                }
            }

            if x > 0 {
                if let Some(other_tile) = self.data.get(&(y, x - 1)) {
                    cells_found += 1;
                    if tile.first_column() == other_tile.last_column() {
                        cells_matching += 1;
                    }
                }
            }

            if self.range_x().contains(&(x + 1)) {
                if let Some(other_tile) = self.data.get(&(y, x + 1)) {
                    cells_found += 1;
                    if tile.last_column() == other_tile.first_column() {
                        cells_matching += 1;
                    }
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

    pub fn contains_tile(&self, tile: &Tile) -> bool {
        self.tile_ids.contains(&(tile.get_number()))
    }

    pub fn contains_index(&self, x: u16, y: u16) -> bool {
        self.data.contains_key(&(y, x))
    }

    pub fn index_in_range(&self, x: i16, y: i16) -> bool {
        if x < 0 || y < 0 {
            false
        } else if !(self.range_x().contains(&(x as u16))) || !(self.range_y().contains(&(y as u16)))
        {
            false
        } else {
            true
        }
    }

    pub fn range_x(&self) -> Range<u16> {
        0..self.size
    }

    pub fn range_y(&self) -> Range<u16> {
        0..self.size
    }

    pub fn is_full(&self) -> bool {
        if self.data.len() == (self.size * self.size) as usize {
            true
        } else {
            false
        }
    }
}

impl fmt::Display for Pattern {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.data.is_empty() {
            return Ok(());
        }

        for y in self.range_y() {
            for x in self.range_x() {
                let idx = (y, x);
                if let Some(tile) = self.data.get(&idx) {
                    write!(f, "{}\t", tile.get_number())?;
                } else {
                    write!(f, "    \t")?;
                }
            }
            writeln!(f, "")?;
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
        let p = Pattern::new(1);
        assert_eq!(format!("{}", p), "");
    }

    #[test]
    fn test_single_tile() {
        let mut p = Pattern::new(11);
        // insert into empty pattern is successful
        p.try_insert(&(Tile::new(1234)), 0, 0).unwrap();
        assert!(format!("{}", p).contains("1234"));
        assert!(p.contains_tile(&(Tile::new(1234))));
        assert!(!(p.contains_tile(&(Tile::new(4)))));

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
        2,
        1,
        "Tile 2:\n\
        #.#\n\
        #..\n\
        #.#",
        "    \t    \t    \t\n    \t1\t2\t\n    \t    \t    \t\n";
        "right"
    )]
    #[test_case(
        "Tile 1:\n\
        #.#\n\
        ..#\n\
        ..#",
        0,
        1,
        "Tile 2:\n\
        #.#\n\
        #..\n\
        #..",
        "    \t    \t    \t\n2\t1\t    \t\n    \t    \t    \t\n";
        "left"
    )]
    #[test_case(
        "Tile 1:\n\
        #.#\n\
        ..#\n\
        ..#",
        1,
        2,
        "Tile 2:\n\
        ..#\n\
        #..\n\
        #.#",
        "    \t    \t    \t\n    \t1\t    \t\n    \t2\t    \t\n";
        "down"
    )]
    #[test_case(
        "Tile 1:\n\
        #.#\n\
        ..#\n\
        ..#",
        1,
        0,
        "Tile 2:\n\
        #..\n\
        #..\n\
        #.#", 
        "    \t2\t    \t\n    \t1\t    \t\n    \t    \t    \t\n"; 
        "up"
    )]
    fn test_try_insert(input_0_0: &str, x: u16, y: u16, new_tile: &str, expected: &str) {
        // intial setup
        let mut p = Pattern::new(3);
        let input_tile: Tile = input_0_0.parse().unwrap();
        p.try_insert(&input_tile, 1, 1).unwrap();

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
    fn test_try_insert_fail(input_0_0: &str, x: u16, y: u16, new_tile: &str) {
        // intial setup
        let mut p = Pattern::new(3);
        let input_tile: Tile = input_0_0.parse().unwrap();
        p.try_insert(&input_tile, 0, 0).unwrap();

        // new tile
        let new_tile: Tile = new_tile.parse().unwrap();
        assert!(p.try_insert(&new_tile, x, y).is_err());
    }
}
