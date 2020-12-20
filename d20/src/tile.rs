use simple_error::{bail, SimpleError};
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq)]
pub struct Tile {
    num: u16,
    data: Vec<Vec<bool>>,
}

impl Tile {
    pub fn new(num: u16) -> Self {
        Tile {
            num,
            data: Vec::new(),
        }
    }

    pub fn get_number(&self) -> u16 {
        self.num
    }

    pub fn check(&self) -> bool {
        let len = self.data.len();

        for line in self.data.iter() {
            if line.len() != len {
                return false;
            }
        }
        true
    }

    pub fn get_row(&self, y: usize) -> Vec<bool> {
        self.data[y].clone()
    }

    pub fn first_row(&self) -> Vec<bool> {
        self.get_row(0)
    }

    pub fn last_row(&self) -> Vec<bool> {
        self.get_row(self.data.len() - 1)
    }

    pub fn get_column(&self, x: usize) -> Vec<bool> {
        self.data.iter().map(|row| row[x]).collect()
    }

    pub fn first_column(&self) -> Vec<bool> {
        self.get_column(0)
    }

    pub fn last_column(&self) -> Vec<bool> {
        // we assume the vecs are same size
        // TODO: be sure
        self.get_column(self.data.len() - 1)
    }

    pub fn flip_x(&self) -> Self {
        Tile {
            num: self.num,
            data: self
                .data
                .iter()
                .map(|row| {
                    (0..row.len())
                        .rev()
                        .map(|source_idx| row[source_idx])
                        .collect()
                })
                .collect(),
        }
    }

    pub fn flip_y(&self) -> Self {
        Tile {
            num: self.num,
            data: self.data.iter().rev().cloned().collect(),
        }
    }

    pub fn rotate_90(&self) -> Self {
        let mut ret = Tile {
            num: self.num,
            data: (0..self.data.len())
                .map(|_| {
                    let mut v = Vec::new();
                    v.resize(self.data.len(), false);
                    v
                })
                .collect(),
        };

        let n = self.data.len();

        for i in 0..n {
            for j in 0..n {
                ret.data[i][j] = self.data[n - j - 1][i];
            }
        }

        ret
    }

    pub fn rotate_180(&self) -> Self {
        self.rotate_90().rotate_90()
    }

    pub fn rotate_270(&self) -> Self {
        self.rotate_90().rotate_90().rotate_90()
    }
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Tile {}:", self.num,)?;

        for row in self.data.iter() {
            for cell in row.iter() {
                write!(
                    f,
                    "{}",
                    match cell {
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
        let mut lines = input.lines().filter(|l| !(l.is_empty()));

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
            let mut linedata: Vec<bool> = Vec::new();

            for (x, ch) in line.chars().enumerate() {
                linedata.push(match ch {
                    '#' => true,
                    '.' => false,
                    _ => bail!("unknown  boolean character {} at x:{} / y:{}", ch, x, y),
                });
            }
            tile.data.push(linedata);
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

    #[test]
    fn test_flip_x() {
        let input = "Tile 123:\n\
            #..\n\
            ##.\n\
            ###\n";

        let tile = input.parse::<Tile>().unwrap().flip_x();

        assert_eq!(
            format!("{}", tile),
            "Tile 123:\n\
            ..#\n\
            .##\n\
            ###\n"
        );
    }

    #[test]
    fn test_flip_y() {
        let input = "Tile 123:\n\
            #..\n\
            ##.\n\
            ###\n";

        let tile = input.parse::<Tile>().unwrap().flip_y();

        assert_eq!(
            format!("{}", tile),
            "Tile 123:\n\
            ###\n\
            ##.\n\
            #..\n"
        );
    }

    #[test]
    fn test_rotate_90() {
        let input = "Tile 123:\n\
            #.#\n\
            #..\n\
            #..\n";

        let tile = input.parse::<Tile>().unwrap().rotate_90();

        assert_eq!(
            format!("{}", tile),
            "Tile 123:\n\
            ###\n\
            ...\n\
            ..#\n"
        );
    }

    #[test]
    fn test_rotate_180() {
        let input = "Tile 123:\n\
            #.#\n\
            #..\n\
            #..\n";

        let tile = input.parse::<Tile>().unwrap().rotate_180();

        assert_eq!(
            format!("{}", tile),
            "Tile 123:\n\
            ..#\n\
            ..#\n\
            #.#\n"
        );
    }

    #[test]
    fn test_rotate_270() {
        let input = "Tile 123:\n\
            #.#\n\
            #..\n\
            #..\n";

        let tile = input.parse::<Tile>().unwrap().rotate_270();

        assert_eq!(
            format!("{}", tile),
            "Tile 123:\n\
            #..\n\
            ...\n\
            ###\n"
        );
    }
}
