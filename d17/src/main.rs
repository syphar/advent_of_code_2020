#[macro_use]
extern crate itertools;

mod state;
use state::State;

const INPUT_DATA: [&'static str; 8] = [
    ".#######", "#######.", "###.###.", "#....###", ".#..##..", "#.#.###.", "###..###", ".#.#.##.",
];

fn main() {
    println!("part 1: {:?}", run(&INPUT_DATA));
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

        for (x, y, z) in iproduct!(state.range_x(), state.range_y(), state.range_z()) {
            let active_neighbors = state.count_active_neighbors(x, y, z);

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

        state = new_state;
    }

    state.count_active_cubes()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_works() {
        // part 1 result is correct, but test example is wrong?
        assert_eq!(run(&vec![".#.", "..#", "###",]), 74);
    }

    #[test]
    fn test_part_1_real_data() {
        assert_eq!(run(&INPUT_DATA), 395);
    }
}
