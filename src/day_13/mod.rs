use winnow::{ascii::*, combinator::*, Parser};

pub type Input = Vec<Machine>;

pub struct Machine {
    a: Pos,
    b: Pos,
    prize: Pos,
}

#[derive(Clone, Copy)]
pub struct Pos {
    x: i64,
    y: i64,
}

fn parse_input(input: &mut &str) -> winnow::PResult<Input> {
    separated(
        0..,
        (
            ("Button A: X+", dec_int, ", Y+", dec_int, line_ending),
            ("Button B: X+", dec_int, ", Y+", dec_int, line_ending),
            ("Prize: X=", dec_int, ", Y=", dec_int, line_ending),
        )
            .map(
                |((_, ax, _, ay, _), (_, bx, _, by, _), (_, px, _, py, _))| Machine {
                    a: Pos { x: ax, y: ay },
                    b: Pos { x: bx, y: by },
                    prize: Pos { x: px, y: py },
                },
            ),
        line_ending,
    )
    .parse_next(input)
}

pub fn input_generator(input: &str) -> Input {
    parse_input.parse(input).unwrap()
}

const A_COST: i64 = 3;
const B_COST: i64 = 1;

fn solve(m: &Machine) -> Option<u64> {
    // consider the (vector) graph for N_A * A + N_B * B
    // which we want to solve equal to P.
    //
    // If we write the equation using matrix multiplication we get
    // |ax bx||na| = |px|
    // |ay by||nb|   |py|
    // so by finding the inverse of the left matrix we can get our solution.

    let discriminant = m.a.x * m.b.y - m.b.x * m.a.y;

    if discriminant == 0 {
        None
    } else {
        let unscaled_inverse = [[m.b.y, -m.b.x], [-m.a.y, m.a.x]];

        let n_a_unscaled = m.prize.x * unscaled_inverse[0][0] + m.prize.y * unscaled_inverse[0][1];
        let n_b_unscaled = m.prize.x * unscaled_inverse[1][0] + m.prize.y * unscaled_inverse[1][1];

        if n_a_unscaled % discriminant == 0 && n_b_unscaled % discriminant == 0 {
            let cost =
                A_COST * (n_a_unscaled / discriminant) + B_COST * (n_b_unscaled / discriminant);
            (cost > 0).then_some(cost as u64)
        } else {
            None
        }
    }
}

pub fn part_1(input: &Input) -> u64 {
    input.iter().flat_map(solve).sum()
}

const OFFSET: i64 = 10000000000000;

pub fn part_2(input: &Input) -> u64 {
    input
        .iter()
        .map(|m| Machine {
            a: m.a,
            b: m.b,
            prize: Pos {
                x: m.prize.x + OFFSET,
                y: m.prize.y + OFFSET,
            },
        })
        .flat_map(|m| solve(&m))
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
        assert_eq!(part_2(&input), 875318608908);
    }

    #[test]
    fn test_my_input() {
        let input = input_generator(include_str!("../../input/2024/day13.txt"));
        assert_eq!(part_1(&input), 31065);
        assert_eq!(part_2(&input), 93866170395343);
    }
}
