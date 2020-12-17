#[macro_use]
extern crate itertools;

mod state;
use state::State;

fn main() {
    let test_data = vec![
        ".#######", "#######.", "###.###.", "#....###", ".#..##..", "#.#.###.", "###..###",
        ".#.#.##.",
    ];
    println!("part 1: {:?}", run(&test_data));
}

fn run(input: &[&str]) -> usize {
    let mut state: State = State::new();

    for y in 0..input.len() {
        let line = &input[y];
        for x in 0..line.len() {
            if line.chars().nth(x).unwrap() == '#' {
                state.set(x as i64, y as i64, 0, true);
            }
        }
    }

    for _ in 1..=6 {
        let mut new_state = state.clone();
        for x in state.range_x() {
            for y in state.range_y() {
                for z in state.range_z() {
                    let active_neighbors = state.count_neighbors_set(x, y, z);

                    if state.get(x, y, z) == true {
                        if !((2..=3).contains(&active_neighbors)) {
                            new_state.set(x, y, z, false);
                        }
                    } else {
                        if active_neighbors == 3 {
                            new_state.set(x, y, z, true);
                        }
                    }
                }
            }
        }

        state = new_state;
    }

    state.count_active_cubes()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_works() {
        assert_eq!(run(&vec![".#.", "..#", "###",]), 112);
    }
}
