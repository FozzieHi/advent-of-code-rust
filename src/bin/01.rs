advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<i32> {
    Some(input.chars().fold(0, |floor, char| match char {
        '(' => floor + 1,
        ')' => floor - 1,
        _ => panic!(),
    }))
}

pub fn part_two(input: &str) -> Option<i32> {
    let mut floor = 0;
    for (index, char) in input.chars().enumerate() {
        match char {
            '(' => floor += 1,
            ')' => {
                floor -= 1;
                if floor < 0 {
                    return Some((index + 1) as i32);
                }
            }
            _ => panic!(),
        }
    }
    panic!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use itertools::Itertools;

    fn test_part<F>(part_fn: F, file_part: u8)
    where
        F: Fn(&str) -> Option<i32>,
    {
        advent_of_code::template::read_file_part("examples", DAY, file_part)
            .lines()
            .for_each(|line| {
                let (input, expected_str) = line.split_whitespace().collect_tuple().unwrap();
                assert_eq!(part_fn(input), Some(expected_str.parse().unwrap()));
            });
    }

    #[test]
    fn test_part_one() {
        test_part(part_one, 1);
    }

    #[test]
    fn test_part_two() {
        test_part(part_two, 2);
    }
}
