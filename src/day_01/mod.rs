use itertools::Itertools;
use winnow::{
    ascii::{dec_uint, line_ending, space1},
    combinator::*,
    Parser,
};

pub struct Input {
    left: Vec<u32>,
    right: Vec<u32>,
}

fn parse_input(input: &mut &str) -> winnow::PResult<Input> {
    let lines: Vec<_> = separated(0.., separated_pair(dec_uint, space1, dec_uint), line_ending)
        .parse_next(input)?;

    Ok(Input {
        left: lines.iter().map(|line| line.0).sorted_unstable().collect(),
        right: lines.iter().map(|line| line.1).sorted_unstable().collect(),
    })
}

pub fn input_generator(input: &str) -> Input {
    parse_input.parse(input.trim_end()).unwrap()
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
