use itertools::Itertools;
use nom::{character::complete::*, multi::*, Parser};

#[derive(Debug)]
pub struct Report {
    nums: Vec<u32>,
}

pub struct Input {
    reports: Vec<Report>,
}

fn parse_report(input: &str) -> nom::IResult<&str, Report> {
    separated_list1(space1, u32)
        .map(|nums| Report { nums })
        .parse(input)
}

fn parse_input(input: &str) -> nom::IResult<&str, Input> {
    separated_list0(line_ending, parse_report)
        .map(|reports| Input { reports })
        .parse(input)
}

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Input {
    let (remaining, result) = parse_input(input).expect("failed to parse input");
    assert!(remaining.trim().is_empty(), "failed to parse entire input");
    result
}

impl Report {
    fn is_safe_with_skip(&self, allow_skip: bool, skip_idx: Option<usize>) -> bool {
        let mut last_sign = None;
        let nums = match (allow_skip, skip_idx) {
            (true, Some(skip_idx)) => self.nums[0..skip_idx]
                .iter()
                .chain(&self.nums[(skip_idx + 1)..]),
            _ => self.nums.iter().chain(&[]),
        };

        for ((l_idx, left), (r_idx, right)) in nums.enumerate().tuple_windows() {
            let diff = (*right as i32) - (*left as i32);
            let sign = diff.signum();
            if (diff == 0 || diff.abs() > 3) || last_sign.is_some_and(|last_sign| last_sign != sign)
            {
                return allow_skip
                    && skip_idx.is_none()
                    && (self.is_safe_with_skip(true, Some(l_idx))
                        || self.is_safe_with_skip(true, Some(r_idx))
                        || self.is_safe_with_skip(true, Some(l_idx.saturating_sub(1))));
            }
            last_sign = Some(sign);
        }
        true
    }

    fn is_safe(&self, allow_skip: bool) -> bool {
        self.is_safe_with_skip(allow_skip, None)
    }
}

fn solve(input: &Input, allow_skip: bool) -> u32 {
    input
        .reports
        .iter()
        .filter(|report| report.is_safe(allow_skip))
        .count() as u32
}

#[aoc(day2, part1)]
pub fn part_1(input: &Input) -> u32 {
    solve(input, false)
}

#[aoc(day2, part2)]
pub fn part_2(input: &Input) -> u32 {
    solve(input, true)
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test() {
        let input = input_generator(indoc! {
            "
            7 6 4 2 1
            1 2 7 8 9
            9 7 6 2 1
            1 3 2 4 5
            8 6 4 4 1
            1 3 6 7 9
            "
        });
        assert_eq!(part_1(&input), 2);
        assert_eq!(part_2(&input), 4);
    }

    #[test]
    fn test_my_input() {
        let input = input_generator(include_str!("../../input/2024/day2.txt"));
        assert_eq!(part_1(&input), 598);
        assert_eq!(part_2(&input), 634);
    }
}