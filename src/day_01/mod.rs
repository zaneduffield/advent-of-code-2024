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
    let left = input.iter().map(|elm| elm.0).sorted().collect_vec();
    let right = input.iter().map(|elm| elm.1).sorted().collect_vec();

    let mut sum = 0;
    let mut r_idx = 0;
    for num in left {
        while matches!(right.get(r_idx), Some(x) if *x < num) {
            r_idx += 1;
        }
        let mut count = 0;
        let mut temp_idx = r_idx;
        while right.get(temp_idx) == Some(&num) {
            count += 1;
            temp_idx += 1;
        }
        sum += num * count;
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
