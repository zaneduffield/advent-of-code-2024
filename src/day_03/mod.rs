use std::sync::LazyLock;

use regex::Regex;

pub type Input = str;

static MUL_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"do\(\)|don't\(\)|mul\((\d{1,3}),(\d{1,3})\)"#).unwrap());

#[aoc(day3, part1)]
pub fn part_1(input: &Input) -> u32 {
    MUL_REGEX
        .captures_iter(input)
        .filter_map(|m| {
            Some(
                m.get(1)?.as_str().parse::<u32>().unwrap()
                    * m.get(2)?.as_str().parse::<u32>().unwrap(),
            )
        })
        .sum()
}

#[aoc(day3, part2)]
pub fn part_2(input: &Input) -> u32 {
    let mut enabled = true;
    MUL_REGEX
        .captures_iter(input)
        .filter_map(|m| {
            let whole = m.get(0).unwrap();
            if whole.as_str() == "do()" {
                enabled = true;
                None
            } else if whole.as_str() == "don't()" {
                enabled = false;
                None
            } else if enabled {
                Some(
                    m.get(1)?.as_str().parse::<u32>().unwrap()
                        * m.get(2)?.as_str().parse::<u32>().unwrap(),
                )
            } else {
                None
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!(part_1(input), 161);
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!(part_2(input), 48);
    }

    #[test]
    fn test_my_input() {
        let input = include_str!("../../input/2024/day3.txt");
        assert_eq!(part_1(input), 166357705);
        assert_eq!(part_2(input), 88811886);
    }
}
