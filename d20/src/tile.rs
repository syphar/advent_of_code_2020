use simple_error::{bail, SimpleError};
use std::convert::TryInto;
use std::fmt;
use std::str::FromStr;

pub enum TileConversion {
    FlipX,
    FlipY,
    Rotate90,
    Rotate180,
    Rotate270,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Tile {
    num: u16,
    data: [[bool; 10]; 10],
}

impl Tile {
    pub fn new(num: u16) -> Self {
        Tile {
            num,
            data: [[false; 10]; 10],
        }
    }

    pub fn get_row(&self, y: usize) -> [bool; 10] {
        self.data[y]
    }

    pub fn get_column(&self, x: usize) -> [bool; 10] {
        self.data
            .iter()
            .map(|row| row[x])
            .collect::<Vec<_>>()
            .as_slice()
            .try_into()
            .unwrap()
    }
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Tile {}:", self.num)?;

        for y in 0..10 {
            for x in 0..10 {
                write!(
                    f,
                    "{}",
                    match self.data[y][x] {
                        true => "#",
                        false => ".",
                    }
                )?;
            }
            writeln!(f, "")?;
        }

        Ok(())
    }
}

impl FromStr for Tile {
    type Err = SimpleError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut lines = input.lines();

        let mut tile: Tile;

        if let Some(line) = lines.next() {
            if !(line.starts_with("Tile ")) {
                bail!("missing 'Tile' in first line");
            }
            if !(line.ends_with(":")) {
                bail!("missing ':' in first line");
            }

            if let Ok(n) = line[5..(line.len() - 1)].parse() {
                tile = Tile::new(n);
            } else {
                bail!("could not parse tile number from '{}'", line);
            }
        } else {
            bail!("no data");
        }

        for (y, line) in lines.enumerate() {
            if y >= 10 {
                break;
            }

            for (x, ch) in line.chars().enumerate() {
                if x >= 10 {
                    break;
                }

                tile.data[y][x] = match ch {
                    '#' => true,
                    '.' => false,
                    _ => bail!("unknown  boolean character {} at x:{} / y:{}", ch, x, y),
                };
            }
        }

        Ok(tile)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(""; "empty")]
    #[test_case("asdf")]
    #[test_case("Tile 1234"; "missing colon")]
    fn test_parse_tile_fail(input: &str) {
        assert!(input.parse::<Tile>().is_err());
    }

    #[test_case("Tile 4321:", Tile::new(4321))]
    fn test_parse_tile_title_ok(input: &str, expected: Tile) {
        assert_eq!(input.parse(), Ok(expected));
    }

    #[test]
    fn test_parse_tile_complete() {
        let input = "Tile 3079:\n\
            #.#.#####.\n\
            .#..######\n\
            ..#.......\n\
            ######....\n\
            ####.#..#.\n\
            .#...#.##.\n\
            #.#####.##\n\
            ..#.###...\n\
            ..#.......\n\
            ..#.###...\n";

        let tile: Tile = input.parse().unwrap();

        assert_eq!(
            tile.data[0],
            [true, false, true, false, true, true, true, true, true, false]
        );
        assert_eq!(
            tile.data[9],
            [false, false, true, false, true, true, true, false, false, false]
        );

        assert_eq!(
            tile.get_row(0),
            [true, false, true, false, true, true, true, true, true, false]
        );

        assert_eq!(
            tile.get_column(0),
            [true, false, false, true, true, false, true, false, false, false]
        );

        assert_eq!(format!("{}", tile), input);
    }
}
