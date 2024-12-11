use fxhash::FxHashMap;
use winnow::{ascii::*, combinator::*, Parser};

pub type Input = Vec<u64>;

fn parse_input(input: &mut &str) -> winnow::PResult<Input> {
    separated(0.., dec_uint::<_, u64, _>, space1).parse_next(input)
}

pub fn input_generator(mut input: &str) -> Input {
    terminated(parse_input, (opt(line_ending), eof))
        .parse_next(&mut input)
        .expect("failed to parse input")
}

fn expanded_len(mut num: u64, repetitions: u8, memoized: &mut FxHashMap<(u64, u8), u64>) -> u64 {
    if let Some(len) = memoized.get(&(num, repetitions)) {
        return *len;
    }

    for rep in 1..=repetitions {
        if num == 0 {
            num = 1;
        } else {
            let digits = num.ilog10() + 1;
            if digits % 2 == 0 {
                let divisor = 10u64.pow(digits / 2);
                let reps_left = repetitions - rep;

                return [num / divisor, num % divisor]
                    .map(|split_num| {
                        // Unfortunately we can't use the Entry API to make this more efficient,
                        // because borrowck won't accept the `memoized` variable being used inside
                        // the closure passed to `or_insert_with`. Polonius would probably solve it.
                        if let Some(len) = memoized.get(&(split_num, reps_left)) {
                            *len
                        } else {
                            let len = expanded_len(split_num, reps_left, memoized);
                            memoized.insert((split_num, reps_left), len);
                            len
                        }
                    })
                    .iter()
                    .sum();
            } else {
                num *= 2024;
            }
        }
    }

    1
}

fn solve(input: &Input, repetitions: u8) -> u64 {
    let mut memo = FxHashMap::default();
    input
        .iter()
        .map(|&num| expanded_len(num, repetitions, &mut memo))
        .sum()
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
