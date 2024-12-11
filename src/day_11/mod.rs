use fxhash::FxHashMap;
use winnow::{ascii::*, combinator::*, Parser};

pub type Input = Vec<u64>;

fn parse_input(input: &mut &str) -> winnow::PResult<Input> {
    separated(0.., dec_uint::<_, u64, _>, space1).parse_next(input)
}

pub fn input_generator(input: &str) -> Input {
    parse_input.parse(input.trim_end()).unwrap()
}

fn solve(input: &Input, repetitions: u8) -> u64 {
    let mut rock_counts: FxHashMap<u64, u64> = input.iter().map(|num| (*num, 1)).collect();
    let mut next_counts = FxHashMap::<u64, u64>::default();

    for _ in 0..repetitions {
        for (num, count) in rock_counts.drain() {
            if num == 0 {
                *next_counts.entry(1).or_default() += count;
            } else {
                let digits = num.ilog10() + 1;
                if digits % 2 == 0 {
                    let divisor = 10u64.pow(digits / 2);

                    *next_counts.entry(num / divisor).or_default() += count;
                    *next_counts.entry(num % divisor).or_default() += count;
                } else {
                    *next_counts.entry(num * 2024).or_default() += count;
                }
            }
        }

        std::mem::swap(&mut rock_counts, &mut next_counts);
    }

    rock_counts.values().sum()
}

pub fn part_1(input: &Input) -> u64 {
    solve(input, 25)
}

pub fn part_2(input: &Input) -> u64 {
    solve(input, 75)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = input_generator("125 17");
        assert_eq!(part_1(&input), 55312);
        assert_eq!(part_2(&input), 65601038650482);
    }

    #[test]
    fn test_my_input() {
        let input = input_generator(include_str!("../../input/2024/day11.txt"));
        assert_eq!(part_1(&input), 209412);
        assert_eq!(part_2(&input), 248967696501656);
    }
}
