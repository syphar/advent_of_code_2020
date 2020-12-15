fn main() {
    let data = vec![2, 0, 6, 12, 1, 3];
    println!("part 1: {:?}", run(&data, 2020));
    println!("part 2: {:?}", run(&data, 30000000));
}

fn run(numbers: &[u64], until: usize) -> u64 {
    let mut n: Vec<u64> = numbers.iter().cloned().collect();

    while n.len() < until {
        let last_number = n.last().unwrap();

        if let Some(already_spoken_at) = n
            .iter()
            .enumerate()
            .rev()
            .skip(1)
            .filter(|(_, &v)| v == *last_number)
            .map(|(i, _)| i)
            .next()
        {
            let last_number_turn = n.len();
            let already_spoken_turn = already_spoken_at + 1;
            let new_number = last_number_turn - already_spoken_turn;
            n.push(new_number as u64);
        } else {
            n.push(0);
        }
    }

    *(n.last().unwrap())
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
    fn part_1_works(input: &[u64], expected: u64) {
        assert_eq!(run(&input, 2020), expected);
    }

    // #[test_case(&[0,3,6], 175594)]
    // #[test_case(&[1,3,2], 2578)]
    // #[test_case(&[2,1,3], 3544142)]
    // #[test_case(&[1,2,3], 261214)]
    // #[test_case(&[2,3,1], 6895259)]
    // #[test_case(&[3,2,1], 18)]
    // #[test_case(&[3,1,2], 362)]
    // fn part_2_works(input: &[u64], expected: u64) {
    //     assert_eq!(run(&input, 30000000), expected);
    // }
}
