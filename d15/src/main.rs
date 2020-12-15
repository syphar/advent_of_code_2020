fn main() {
    println!("part 1: {:?}", part_1(&vec![2, 0, 6, 12, 1, 3]));
    // println!("part 2: {:?}", part_2(input.iter()));
}

fn part_1(numbers: &[u64]) -> u64 {
    let mut n: Vec<u64> = numbers.iter().cloned().collect();

    while n.len() < 2020 {
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
        assert_eq!(part_1(&input), expected);
    }
}
