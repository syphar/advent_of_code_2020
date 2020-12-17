#[macro_use]
extern crate itertools;
use std::cmp;
use std::collections::HashSet;
use std::ops::RangeInclusive;

const INPUT_DATA: [&'static str; 8] = [
    ".#######", "#######.", "###.###.", "#....###", ".#..##..", "#.#.###.", "###..###", ".#.#.##.",
];

fn main() {
    println!("part 1: {:?}", run(&INPUT_DATA));
    println!("part 2: {:?}", run_2(&INPUT_DATA));
}

type Space = HashSet<(i64, i64, i64)>;
type Space4 = HashSet<(i64, i64, i64, i64)>;

fn find_range(space: &Space) -> RangeInclusive<i64> {
    let min = space
        .iter()
        .map(|idx| cmp::min(cmp::min(idx.0, idx.1), idx.2))
        .min()
        .unwrap();
    let max = space
        .iter()
        .map(|idx| cmp::max(cmp::max(idx.0, idx.1), idx.2))
        .max()
        .unwrap();
    (min - 1)..=(max + 1)
}

fn find_range_4(space: &Space4) -> RangeInclusive<i64> {
    let min = space
        .iter()
        .map(|idx| cmp::min(cmp::min(cmp::min(idx.0, idx.1), idx.2), idx.3))
        .min()
        .unwrap();
    let max = space
        .iter()
        .map(|idx| cmp::max(cmp::max(cmp::max(idx.0, idx.1), idx.2), idx.3))
        .max()
        .unwrap();
    (min - 1)..=(max + 1)
}

fn count_active_neighbors(space: &Space, idx: (i64, i64, i64)) -> usize {
    let mut count = 0;
    for (dx, dy, dz) in iproduct!(-1..=1, -1..=1, -1..=1) {
        if dx == 0 && dy == 0 && dz == 0 {
            continue;
        }
        if space.contains(&(idx.0 + dx, idx.1 + dy, idx.2 + dz)) {
            count += 1;
        }
    }
    count
}

fn count_active_neighbors_4(space: &Space4, idx: (i64, i64, i64, i64)) -> usize {
    let mut count = 0;
    for (dx, dy, dz, dw) in iproduct!(-1..=1, -1..=1, -1..=1, -1..=1) {
        if dx == 0 && dy == 0 && dz == 0 && dw == 0 {
            continue;
        }
        if space.contains(&(idx.0 + dx, idx.1 + dy, idx.2 + dz, idx.3 + dw)) {
            count += 1;
        }
    }
    count
}

fn run(input: &[&str]) -> usize {
    let mut state: Space = Space::new();

    for y in 0..input.len() {
        let line = &input[y];
        for x in 0..line.len() {
            if line.chars().nth(x).unwrap() == '#' {
                state.insert((x as i64, y as i64, 0));
            }
        }
    }

    for _ in 1..=6 {
        let mut new_state = state.clone();

        let range = find_range(&state);

        for idx in iproduct!(range.clone(), range.clone(), range.clone()) {
            let active_neighbors = count_active_neighbors(&state, idx);

            if state.contains(&idx) {
                if !((2..=3).contains(&active_neighbors)) {
                    new_state.remove(&idx);
                }
            } else {
                if active_neighbors == 3 {
                    new_state.insert(idx);
                }
            }
        }

        state = new_state;
    }

    state.len()
}
fn run_2(input: &[&str]) -> usize {
    let mut state: Space4 = Space4::new();

    for y in 0..input.len() {
        let line = &input[y];
        for x in 0..line.len() {
            if line.chars().nth(x).unwrap() == '#' {
                state.insert((x as i64, y as i64, 0, 0));
            }
        }
    }

    for _ in 1..=6 {
        let mut new_state = state.clone();

        let range = find_range_4(&state);

        for idx in iproduct!(range.clone(), range.clone(), range.clone(), range.clone()) {
            let active_neighbors = count_active_neighbors_4(&state, idx);

            if state.contains(&idx) {
                if !((2..=3).contains(&active_neighbors)) {
                    new_state.remove(&idx);
                }
            } else {
                if active_neighbors == 3 {
                    new_state.insert(idx);
                }
            }
        }

        state = new_state;
    }

    state.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_works() {
        // part 1 result is correct, but test example is wrong?
        assert_eq!(run(&vec![".#.", "..#", "###",]), 112);
    }

    #[test]
    fn test_part_1_real_data() {
        assert_eq!(run(&INPUT_DATA), 395);
    }

    //     #[test]
    //     fn test_part_2_real_data() {
    //         assert_eq!(run_2(&INPUT_DATA), 2296);
    //     }
}
