use nom::{bytes::complete::*, character::complete::*, multi::*, sequence::separated_pair};

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

fn satisfiable_rec<const N: usize>(
    eq: &Eq,
    next_pos: usize,
    total: i64,
    ops: [fn(i64, i64) -> i64; N],
) -> bool {
    (next_pos >= eq.1.len() && eq.0 == total)
        || next_pos < eq.1.len()
            && ops
                .iter()
                .any(|op| satisfiable_rec(eq, next_pos + 1, op(total, eq.1[next_pos]), ops))
}

fn satisfiable(eq: &Eq) -> bool {
    satisfiable_rec(
        eq,
        1,
        eq.1[0],
        [|total, next| total * next, |total, next| total + next],
    )
}

#[aoc(day7, part1)]
pub fn part_1(input: &Input) -> i64 {
    input
        .equations
        .iter()
        .filter(|eq| satisfiable(eq))
        .map(|eq| eq.0)
        .sum()
}

fn satisfiable_part2(eq: &Eq) -> bool {
    satisfiable_rec(
        eq,
        1,
        eq.1[0],
        [
            |total, next| total * next,
            |total, next| total + next,
            |total, next| total * 10i64.pow(((next as f64).log(10.0) + 1.0) as u32) + next,
        ],
    )
}

#[aoc(day7, part2)]
pub fn part_2(input: &Input) -> i64 {
    input
        .equations
        .iter()
        .filter(|eq| satisfiable_part2(eq))
        .map(|eq| eq.0)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

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
