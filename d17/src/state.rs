use std::collections::HashMap;
use std::fmt;
use std::ops::RangeInclusive;

#[derive(Clone)]
pub struct State {
    data: HashMap<(i64, i64, i64), bool>,
}

impl State {
    pub fn new() -> Self {
        State {
            data: HashMap::new(),
        }
    }

    pub fn set(&mut self, x: i64, y: i64, z: i64, value: bool) {
        self.data.insert((x, y, z), value);
    }

    pub fn get(&self, x: i64, y: i64, z: i64) -> bool {
        let idx = (x, y, z);

        if self.data.contains_key(&idx) {
            self.data.get(&idx).unwrap().clone()
        } else {
            false
        }
    }

    pub fn count_active_cubes(&self) -> usize {
        self.data.values().filter(|&v| *v == true).count()
    }

    pub fn count_neighbors_set(&self, x: i64, y: i64, z: i64) -> i64 {
        let mut count = 0;
        for (dx, dy, dz) in iproduct!(-1..=1, -1..=1, -1..=1) {
            if x == 0 && y == 0 && z == 0 {
                continue;
            }
            if self.get(x + dx, y + dy, z + dz) == true {
                count += 1;
            }
        }
        count
    }

    fn keys_to_range(&self, all: &Vec<i64>) -> RangeInclusive<i64> {
        if all.is_empty() {
            0..=0
        } else {
            let min = all.iter().cloned().min().unwrap();
            let max = all.iter().cloned().max().unwrap();

            (min - 1)..=(max + 1)
        }
    }

    pub fn range_x(&self) -> RangeInclusive<i64> {
        self.keys_to_range(&self.data.keys().map(|(x, _, _)| x).cloned().collect())
    }
    pub fn range_y(&self) -> RangeInclusive<i64> {
        self.keys_to_range(&self.data.keys().map(|(_, y, _)| y).cloned().collect())
    }
    pub fn range_z(&self) -> RangeInclusive<i64> {
        self.keys_to_range(&self.data.keys().map(|(_, _, z)| z).cloned().collect())
    }
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for z in self.range_z() {
            writeln!(f, "\nz={}", z)?;

            for y in self.range_y() {
                for x in self.range_x() {
                    if self.get(x, y, z) == true {
                        write!(f, "#")?;
                    } else {
                        write!(f, ".")?;
                    }
                }
                writeln!(f, "")?;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_max() {
        let mut state = State::new();
        state.set(-2, -4, -5, true);
        state.set(2, 4, 5, true);

        assert_eq!(state.range_x(), -3..=3);
        assert_eq!(state.range_y(), -5..=5);
        assert_eq!(state.range_z(), -6..=6);
    }

    #[test]
    fn test_count_neighbors() {
        let mut state = State::new();

        assert_eq!(state.count_neighbors_set(0, 0, 0), 0);

        state.set(0, 0, 0, true);
        assert_eq!(state.count_neighbors_set(0, 0, 0), 0);

        assert_eq!(state.count_neighbors_set(1, 0, 0), 1);
        assert_eq!(state.count_neighbors_set(2, 0, 0), 0);
    }

    #[test]
    fn test_active_cubes() {
        let mut state = State::new();

        assert_eq!(state.count_active_cubes(), 0);

        state.set(0, 0, 0, true);
        assert_eq!(state.count_active_cubes(), 1);

        state.set(0, 0, 0, true);
        assert_eq!(state.count_active_cubes(), 1);

        state.set(1, 0, 0, true);
        assert_eq!(state.count_active_cubes(), 2);
    }
}
