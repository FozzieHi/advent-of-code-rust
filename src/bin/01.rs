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
    use advent_of_code::{get_part_line};

    #[test]
    fn test_part_one() {
        get_part_line(DAY, 1).iter().for_each(|(input, expected)| {
            assert_eq!(part_one(input), Some(*expected as i32));
        });
    }

    #[test]
    fn test_part_two() {
        get_part_line(DAY, 2).iter().for_each(|(input, expected)| {
            assert_eq!(part_two(input), Some(*expected as i32));
        });
    }
}
