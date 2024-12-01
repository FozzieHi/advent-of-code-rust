advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let parsed: Vec<(i32, i32)> = input
        .lines()
        .filter_map(|line| {
            let mut parts = line.split_whitespace();
            match (parts.next(), parts.next()) {
                (Some(a), Some(b)) => {
                    let num_a = a.parse::<i32>().ok()?;
                    let num_b = b.parse::<i32>().ok()?;
                    Some((num_a, num_b))
                }
                _ => None,
            }
        })
        .collect();

    let (mut list_a, mut list_b): (Vec<i32>, Vec<i32>) = parsed.into_iter().unzip();

    list_a.sort_unstable();
    list_b.sort_unstable();

    let total: i32 = list_a.iter().zip(list_b).map(|(a, b)| (a - b).abs()).sum();

    u32::try_from(total).ok()
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
