use winnow::{ascii::*, combinator::*, token::*, Parser};

pub type Input = Vec<Machine>;

pub struct Machine {
    a: (u32, u32),
    b: (u32, u32),
    prize: (u32, u32),
}

fn parse_input(input: &mut &str) -> winnow::PResult<Input> {
    separated(
        0..,
        (
            ("Button A: X+", dec_uint, ", Y+", dec_uint, line_ending),
            ("Button B: X+", dec_uint, ", Y+", dec_uint, line_ending),
            ("Prize: X=", dec_uint, ", Y=", dec_uint, line_ending),
        )
            .map(
                |((_, ax, _, ay, _), (_, bx, _, by, _), (_, px, _, py, _))| Machine {
                    a: (ay, ax),
                    b: (by, bx),
                    prize: (py, px),
                },
            ),
        line_ending,
    )
    .parse_next(input)
}

pub fn input_generator(input: &str) -> Input {
    parse_input.parse(input).unwrap()
}

const A_COST: u32 = 3;
const B_COST: u32 = 1;

fn solve(m: &Machine) -> Option<u32> {
    for n_a in 0..=100 {
        for n_b in 0..=100 {
            if n_a * m.a.0 + n_b * m.b.0 == m.prize.0 && n_a * m.a.1 + n_b * m.b.1 == m.prize.1 {
                let cost = A_COST * n_a + B_COST * n_b;
                return Some(cost);
            }
        }
    }

    None
}

pub fn part_1(input: &Input) -> u32 {
    input.iter().flat_map(solve).sum()
}

const OFFSET: u64 = 10000000000000;

pub fn part_2(input: &Input) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test() {
        let input = input_generator(indoc! {
            "
            Button A: X+94, Y+34
            Button B: X+22, Y+67
            Prize: X=8400, Y=5400

            Button A: X+26, Y+66
            Button B: X+67, Y+21
            Prize: X=12748, Y=12176

            Button A: X+17, Y+86
            Button B: X+84, Y+37
            Prize: X=7870, Y=6450

            Button A: X+69, Y+23
            Button B: X+27, Y+71
            Prize: X=18641, Y=10279
            "
        });
        assert_eq!(part_1(&input), 480);
        // assert_eq!(part_2(&input),);
    }

    #[test]
    fn test_my_input() {
        let input = input_generator(include_str!("../../input/2024/day13.txt"));
        // assert_eq!(part_1(&input), );
        // assert_eq!(part_2(&input),);
    }
}
