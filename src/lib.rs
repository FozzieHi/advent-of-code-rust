use crate::template::Day;
use itertools::Itertools;

pub mod template;

// Use this file to add helper functions and additional modules.
pub fn get_part_line(day: Day, file_part: u8) -> Vec<(String, isize)> {
    template::read_file_part("examples", day, file_part)
        .lines()
        .map(|line| {
            line.split_whitespace()
                .collect_tuple::<(_, _)>()
                .map(|(input, expected_str)| (input.to_string(), expected_str.parse().unwrap()))
                .unwrap()
        })
        .collect()
}
