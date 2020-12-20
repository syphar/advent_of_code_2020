use simple_error::{bail, SimpleResult};
use std::fs::File;
use std::io::{BufRead, BufReader};

mod pattern;
use pattern::Pattern;
mod tile;
use tile::Tile;

fn main() {
    // println!(
    //     "part 1: {:?}",
    //     part_1(
    //         BufReader::new(File::open("input_messages.txt").unwrap())
    //             .lines()
    //             .map(|line| line.unwrap().to_string()),
    //         &rule,
    //     )
    // );
}

fn try_children(pattern: &Pattern, tiles: &Vec<Tile>, x: u16, y: u16) -> SimpleResult<Pattern> {
    println!("try insert into {}/{}\n{}", x, y, pattern);

    if pattern.is_full() {
        Ok(pattern.clone())
    } else {
        for tile in tiles {
            if pattern.contains_tile(&tile) {
                continue;
            }
            if let Ok(p) = try_insert(&pattern, &tile, x, y) {
                // try to insert tiles into the surounding empty cells
                println!("\t did insert tile {}", tile.get_number());

                let diff: Vec<(i16, i16)> = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];

                let mut new_pattern = p.clone();

                for (dy, dx) in diff {
                    let check_x: i16 = (x as i16) + dx;
                    let check_y: i16 = (y as i16) + dy;

                    if !(new_pattern.index_in_range(check_x, check_y)) {
                        continue;
                    }

                    if !(new_pattern.contains_index(check_x as u16, check_y as u16)) {
                        if let Ok(pp) =
                            try_children(&new_pattern, &tiles, check_x as u16, check_y as u16)
                        {
                            new_pattern = pp;
                        } else {
                            bail!("could not fill in empty cells!");
                        }
                    }
                }
                return Ok(new_pattern);
            }
        }
        bail!("no matching remaining tiles found");
    }
}

fn try_insert(pattern: &Pattern, new_tile: &Tile, x: u16, y: u16) -> SimpleResult<Pattern> {
    let mut p = pattern.clone();

    if let Ok(_) = p.try_insert(new_tile, x, y) {
        return Ok(p);
    } else if let Ok(_) = p.try_insert(&new_tile.flip_x(), x, y) {
        return Ok(p);
    } else if let Ok(_) = p.try_insert(&new_tile.flip_y(), x, y) {
        return Ok(p);
    } else if let Ok(_) = p.try_insert(&new_tile.rotate_90(), x, y) {
        return Ok(p);
    } else if let Ok(_) = p.try_insert(&new_tile.rotate_180(), x, y) {
        return Ok(p);
    } else if let Ok(_) = p.try_insert(&new_tile.rotate_270(), x, y) {
        return Ok(p);
    } else {
        bail!(
            "tile {} doesn't fit at x {} / y {}",
            new_tile.get_number(),
            x,
            y
        );
    }
}

fn part_1(tiles: &Vec<Tile>, size: u16) -> SimpleResult<u64> {
    let p = try_children(&Pattern::new(size), &tiles, 0, 0)?;
    println!("PATTERN: {}", p);

    Ok(0)
}

fn read_tiles(lines: impl Iterator<Item = String>) -> SimpleResult<Vec<Tile>> {
    let mut result: Vec<Tile> = Vec::new();
    let mut current_lines = Vec::new();
    for line in lines {
        if line.trim().is_empty() {
            result.push(current_lines.join("\n").parse()?);
            current_lines.clear();
        }

        current_lines.push(line);
    }

    if !(current_lines.is_empty()) {
        result.push(current_lines.join("\n").parse()?);
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_input() -> impl Iterator<Item = String> {
        BufReader::new(File::open("input_test.txt").unwrap())
            .lines()
            .map(|line| line.unwrap().to_string())
    }

    #[test]
    fn test_read_tiles() {
        let tiles = read_tiles(test_input()).unwrap();
        assert_eq!(tiles.len(), 9);
    }

    #[test]
    fn part_1_works() {
        let tiles = read_tiles(test_input()).unwrap();
        assert_eq!(part_1(&tiles, 3), Ok(20899048083289));
    }
}
