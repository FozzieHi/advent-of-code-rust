advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<i32> {
    let mut count = 0;
    input.lines().for_each(|line| {
        let mut parts = line.split('x').map(|part| part.parse::<i32>().unwrap());
        let (length, width, height) = (
            parts.next().unwrap(),
            parts.next().unwrap(),
            parts.next().unwrap(),
        );
        let surface_area = 2 * length * width + 2 * width * height + 2 * height * length;
        let smallest_side_area = *[length * width, width * height, height * length]
            .iter()
            .min()
            .unwrap();
        count += surface_area + smallest_side_area;
    });
    Some(count)
}

pub fn part_two(input: &str) -> Option<i32> {
    let mut count = 0;
    input.lines().for_each(|line| {
        let mut parts = line.split('x').map(|part| part.parse::<i32>().unwrap());
        let (length, width, height) = (
            parts.next().unwrap(),
            parts.next().unwrap(),
            parts.next().unwrap(),
        );
        let smallest_face_perimeter = 2 * *[length + width, width + height, height + length]
            .iter()
            .min()
            .unwrap();
        let volume = length * width * height;
        count += smallest_face_perimeter + volume;
    });
    Some(count)
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_of_code::get_part_line;

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
