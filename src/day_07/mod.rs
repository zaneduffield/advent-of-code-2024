use nom::{bytes::complete::*, character::complete::*, multi::*, sequence::separated_pair};
use num::Integer;

pub struct Input {
    equations: Vec<Eq>,
}

pub type Eq = (i64, Vec<i64>);

fn parse_input(input: &str) -> nom::IResult<&str, Input> {
    let (input, equations) = separated_list0(
        line_ending,
        separated_pair(i64, tag(": "), separated_list1(space1, i64)),
    )(input)?;
    Ok((input, Input { equations }))
}

#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> Input {
    let (remaining, result) = parse_input(input).expect("failed to parse input");
    assert!(remaining.trim().is_empty(), "failed to parse entire input");
    result
}

// By working backwards from the end we are able to prune the search tree because of divisibility constraints.
// Specifically, if the last number doesn't divide the goal number, or (in the case of part 2)
// the goal number doesn't 'end with' the last number, then we know that we've hit a dead end.
// If we worked forward from the start, we would need to go all the way to the end to know whether a sequence
// of operations results in the right number.
fn satisfiable_rev<const N: usize>(
    eq: &Eq,
    next_pos: usize,
    goal: i64,
    ops: [fn(i64, i64) -> Option<i64>; N],
) -> bool {
    if next_pos == 0 {
        return eq.1[0] == goal;
    };

    ops.iter().any(|op| {
        op(goal, eq.1[next_pos])
            .is_some_and(|next_goal| satisfiable_rev(eq, next_pos - 1, next_goal, ops))
    })
}

fn remove_trailing_num(num: i64, on: i64) -> Option<i64> {
    let digits = ((on as f64).log(10.0) + 1.0) as u32;

    let (quot, rem) = num.div_rem(&10i64.pow(digits));
    (rem == on).then_some(quot)
}

fn try_div(num: i64, denom: i64) -> Option<i64> {
    let (quot, rem) = num.div_rem(&denom);
    (rem == 0).then_some(quot)
}

pub fn solve<const N: usize>(input: &Input, ops: [fn(i64, i64) -> Option<i64>; N]) -> i64 {
    input
        .equations
        .iter()
        .filter(|eq| satisfiable_rev(eq, eq.1.len() - 1, eq.0, ops))
        .map(|eq| eq.0)
        .sum()
}

#[aoc(day7, part1)]
pub fn part_1(input: &Input) -> i64 {
    solve(input, [try_div, |goal, next| Some(goal - next)])
}

#[aoc(day7, part2)]
pub fn part_2(input: &Input) -> i64 {
    solve(
        input,
        [remove_trailing_num, try_div, |goal, next| Some(goal - next)],
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_remove_trailing_num() {
        assert_eq!(remove_trailing_num(12345, 6), None);
        assert_eq!(remove_trailing_num(12345, 5), Some(1234));
        assert_eq!(remove_trailing_num(12345, 45), Some(123));
        assert_eq!(remove_trailing_num(12345, 44), None);
        assert_eq!(remove_trailing_num(12345, 46), None);
        assert_eq!(remove_trailing_num(12345, 345), Some(12));
    }

    #[test]
    fn test() {
        let input = input_generator(indoc! {
            "
            190: 10 19
            3267: 81 40 27
            83: 17 5
            156: 15 6
            7290: 6 8 6 15
            161011: 16 10 13
            192: 17 8 14
            21037: 9 7 18 13
            292: 11 6 16 20
            "
        });
        assert_eq!(part_1(&input), 3749);
        assert_eq!(part_2(&input), 11387);
    }

    #[test]
    fn test_my_input() {
        let input = input_generator(include_str!("../../input/2024/day7.txt"));
        assert_eq!(part_1(&input), 882304362421);
        assert_eq!(part_2(&input), 145149066755184);
    }
}
