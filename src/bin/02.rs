advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<i32> {
    let safe_total = input
        .lines()
        .filter(|line| {
            let numbers: Vec<u32> = line
                .split_whitespace()
                .filter_map(|entry| entry.parse::<u32>().ok())
                .collect();

            if numbers.len() <= 1 {
                return true;
            }

            let is_increasing = numbers[1] > numbers[0];
            numbers.windows(2).all(|window| {
                let prev = window[0];
                let current = window[1];

                if current == prev {
                    return false;
                }

                if (current as i32 - prev as i32).abs() > 3 {
                    return false;
                }

                if (is_increasing && current < prev) || (!is_increasing && current > prev) {
                    return false;
                }

                true
            })
        })
        .count() as i32;

    Some(safe_total)
}

pub fn part_two(_input: &str) -> Option<i32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
