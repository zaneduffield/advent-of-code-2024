use winnow::{
    branch::alt,
    bytes::{one_of, tag, take_till1},
    character::dec_uint,
    combinator::opt,
    Parser,
};

pub type Input = Vec<Instruction>;

pub enum Instruction {
    Do,
    Dont,
    Mul(u32, u32),
}

fn parse_instruction(input: &str) -> winnow::IResult<&str, Instruction> {
    alt((
        tag("do()").map(|_| Instruction::Do),
        tag("don't()").map(|_| Instruction::Dont),
        (tag("mul("), dec_uint, one_of(','), dec_uint, one_of(')'))
            .map(|(_, left, _, right, _)| Instruction::Mul(left, right)),
    ))
    .parse_next(input)
}

fn take_until_instruction(input: &str) -> winnow::IResult<&str, &str> {
    opt(take_till1("dm"))
        .parse_next(input)
        .map(|(rem, out)| (rem, out.unwrap_or("")))
}

// regex would be easier, but this is faster
fn parse_input(mut input: &str) -> winnow::IResult<&str, Input> {
    let mut instructions = vec![];
    while let Ok((next_input, _)) = take_until_instruction(input) {
        input = next_input;
        if let Ok((next_input, inst)) = parse_instruction(input) {
            input = next_input;
            instructions.push(inst);
        } else if input.is_empty() {
            break;
        } else {
            input = &input[1..];
        }
    }

    Ok(("", instructions))
}

pub fn input_generator(input: &str) -> Input {
    let (remaining, result) = parse_input(input).expect("failed to parse input");
    assert!(remaining.trim().is_empty(), "failed to parse entire input");
    result
}

pub fn part_1(input: &Input) -> u32 {
    input
        .iter()
        .map(|m| match m {
            Instruction::Mul(left, right) => *left * *right,
            _ => 0,
        })
        .sum()
}

pub fn part_2(input: &Input) -> u32 {
    let mut enabled = true;
    input
        .iter()
        .map(|m| match m {
            Instruction::Mul(left, right) if enabled => *left * *right,
            Instruction::Do => {
                enabled = true;
                0
            }
            Instruction::Dont => {
                enabled = false;
                0
            }
            Instruction::Mul(_, _) => 0,
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = input_generator(
            "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))",
        );
        assert_eq!(part_1(&input), 161);
        let input = input_generator(
            "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))",
        );
        assert_eq!(part_2(&input), 48);
    }

    #[test]
    fn test_my_input() {
        let input = input_generator(include_str!("../../input/2024/day3.txt"));
        assert_eq!(part_1(&input), 166357705);
        assert_eq!(part_2(&input), 88811886);
    }
}
