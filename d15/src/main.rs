use std::collections::HashMap;

fn main() {
    let data = vec![2, 0, 6, 12, 1, 3];
    println!("part 1: {:?}", run(&data, 2020));
    println!("part 2: {:?}", run(&data, 30000000));
}

fn run(numbers: &[usize], until: usize) -> usize {
    let mut last_steps: HashMap<usize, usize> = numbers
        .iter()
        .enumerate()
        .map(|(i, &v)| (v, i + 1))
        .collect();

    let mut last_number_spoken: usize = *(numbers.last().unwrap());
    last_steps.remove(&last_number_spoken);

    for step in (numbers.len() + 1)..=until {
        // println!("\nstep {}", step);
        // println!("last number spoken: {}", last_number_spoken);
        // println!("steps before {:?}", last_steps);

        if let Some(last_step) = last_steps.get(&last_number_spoken) {
            // println!("was already spoken at step {}", last_step);

            let new_number = (step - 1) - last_step;
            // println!("new number: {}", new_number);

            last_steps.insert(last_number_spoken, step - 1);
            last_number_spoken = new_number;
        } else {
            // println!("was first time, putting zero in");
            last_steps.insert(last_number_spoken, step - 1);
            last_number_spoken = 0;
        }
    }

    last_number_spoken
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(&[0,3,6], 436)]
    #[test_case(&[1,3,2], 1)]
    #[test_case(&[2,1,3], 10)]
    #[test_case(&[1,2,3], 27)]
    #[test_case(&[2,3,1], 78)]
    #[test_case(&[3,2,1], 438)]
    #[test_case(&[3,1,2], 1836)]
    fn part_1_works(input: &[usize], expected: usize) {
        assert_eq!(run(&input, 2020), expected);
    }

    #[test_case(&[0,3,6], 175594)]
    #[test_case(&[1,3,2], 2578)]
    #[test_case(&[2,1,3], 3544142)]
    #[test_case(&[1,2,3], 261214)]
    #[test_case(&[2,3,1], 6895259)]
    #[test_case(&[3,2,1], 18)]
    #[test_case(&[3,1,2], 362)]
    fn part_2_works(input: &[usize], expected: usize) {
        assert_eq!(run(&input, 30000000), expected);
    }
}
