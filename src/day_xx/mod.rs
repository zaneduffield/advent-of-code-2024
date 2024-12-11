use winnow::{
    ascii::*,
    token::*,
    combinator::*,
    Parser,
};

pub struct Input {

}

fn parse_input(input: &mut &str) -> winnow::PResult<Input> {
    todo!()
}

pub fn input_generator(mut input: &str) -> Input {
    terminated(parse_input, eof)
        .parse_next(&mut input)
        .expect("failed to parse input")
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
