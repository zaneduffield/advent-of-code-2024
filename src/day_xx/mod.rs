use winnow::{
    ascii::*,
    token::*,
    combinator::*,
    Parser,
};

pub struct Input {

}

fn parse_input(input: &mut &str) -> winnow::PResult<Input> {
    Ok(
        Input {
        },
    )
}

pub fn input_generator(mut input: &str) -> Input {
    let result = parse_input(&mut input).expect("failed to parse input");
    assert!(input.trim().is_empty(), "failed to parse entire input");
    result
}

pub fn part_1(input: &Input) -> u32 {
    0
}

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
            "
        });
        assert_eq!(part_1(&input), );
        // assert_eq!(part_2(&input),);
    }

    #[test]
    fn test_my_input() {
        let input = input_generator(include_str!("../../input/2024/dayxx.txt"));
        // assert_eq!(part_1(&input), );
        // assert_eq!(part_2(&input),);
    }
}
