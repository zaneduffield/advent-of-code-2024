use itertools::Itertools;
use nom::{character::complete::*, multi::*, sequence::separated_pair};

pub struct Input {
    left: Vec<u32>,
    right: Vec<u32>,
}

fn parse_input(input: &str) -> nom::IResult<&str, Input> {
    let (input, lines) = separated_list0(line_ending, separated_pair(u32, space1, u32))(input)?;

    Ok((
        input,
        Input {
            left: lines.iter().map(|line| line.0).sorted_unstable().collect(),
            right: lines.iter().map(|line| line.1).sorted_unstable().collect(),
        },
    ))
}

pub fn input_generator(input: &str) -> Input {
    let (remaining, result) = parse_input(input).expect("failed to parse input");
    assert!(remaining.trim().is_empty(), "failed to parse entire input");
    result
}

pub fn part_1(input: &Input) -> u32 {
    input
        .left
        .iter()
        .zip(&input.right)
        .map(|(left, right)| left.abs_diff(*right))
        .sum()
}

pub fn part_2(input: &Input) -> u32 {
    let mut right = input.right.iter().peekable();

    let mut sum = 0;
    for l_num in &input.left {
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
