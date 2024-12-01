use std::cmp::Ordering;

use itertools::Itertools;
use nom::{character::complete::*, multi::*, sequence::separated_pair};

pub type Input = Vec<(u32, u32)>;

fn parse_input(input: &str) -> nom::IResult<&str, Input> {
    separated_list0(line_ending, separated_pair(u32, space1, u32))(input)
}

#[aoc_generator(day01)]
pub fn input_generator(input: &str) -> Input {
    let (remaining, result) = parse_input(input).expect("failed to parse input");
    assert!(remaining.trim().is_empty(), "failed to parse entire input");
    result
}

#[aoc(day01, part1)]
pub fn part_1(input: &Input) -> u32 {
    input
        .iter()
        .map(|elm| elm.0)
        .sorted()
        .zip(input.iter().map(|elm| elm.1).sorted())
        .map(|(left, right)| left.abs_diff(right))
        .sum()
}

#[aoc(day01, part2)]
pub fn part_2(input: &Input) -> u32 {
    let left = input.iter().map(|elm| elm.0).sorted();
    let mut right = input.iter().map(|elm| elm.1).sorted().peekable();

    let mut sum = 0;
    for l_num in left {
        let _ = right.peeking_take_while(|r_num| *r_num < l_num).last();
        sum += l_num * right.clone().take_while(|r_num| *r_num == l_num).count() as u32;
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test() {
        let input = input_generator(indoc! {
            "
            3   4
            4   3
            2   5
            1   3
            3   9
            3   3
            "
        });
        assert_eq!(part_1(&input), 11);
        assert_eq!(part_2(&input), 31);
    }

    #[test]
    fn test_my_input() {
        let input = input_generator(include_str!("../../input/2024/day1.txt"));
        assert_eq!(part_1(&input), 1580061);
        assert_eq!(part_2(&input), 23046913);
    }
}
