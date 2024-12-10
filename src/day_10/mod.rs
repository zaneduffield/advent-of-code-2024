use fxhash::FxHashSet;

pub type Input<'a> = Vec<&'a [u8]>;

pub fn input_generator(input: &str) -> Input {
    input.lines().map(|line| line.as_bytes()).collect()
}

fn count_trails_from(
    (y, x): (isize, isize),
    val: u8,
    input: &Input,
    reached: &mut FxHashSet<(isize, isize)>,
    count_all_paths: bool,
) -> u32 {
    [(-1, 0), (0, 1), (1, 0), (0, -1)]
        .iter()
        .map(|&(dy, dx)| {
            let (y2, x2) = (y + dy as isize, x + dx as isize);
            match input.get(y2 as usize).and_then(|row| row.get(x2 as usize)) {
                Some(&val2) if val2 == val + 1 => {
                    if val2 == b'9' {
                        if count_all_paths {
                            1
                        } else if reached.contains(&(y2, x2)) {
                            0
                        } else {
                            reached.insert((y2, x2));
                            1
                        }
                    } else {
                        count_trails_from((y2, x2), val2, input, reached, count_all_paths)
                    }
                }
                _ => 0,
            }
        })
        .sum()
}

pub fn solve(input: &Input, count_all_paths: bool) -> u32 {
    let mut count = 0;
    let mut reached = FxHashSet::default();
    for (y, row) in input.iter().enumerate() {
        for (x, col) in row.iter().enumerate() {
            if *col == b'0' {
                reached.clear();
                count += count_trails_from(
                    (y as isize, x as isize),
                    *col,
                    input,
                    &mut reached,
                    count_all_paths,
                );
            }
        }
    }
    count
}

pub fn part_1(input: &Input) -> u32 {
    solve(input, false)
}

pub fn part_2(input: &Input) -> u32 {
    solve(input, true)
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test() {
        let input = input_generator(indoc! {
            "
            89010123
            78121874
            87430965
            96549874
            45678903
            32019012
            01329801
            10456732
            "
        });
        assert_eq!(part_1(&input), 36);
        assert_eq!(part_2(&input), 81);
    }

    #[test]
    fn test_my_input() {
        let input = input_generator(include_str!("../../input/2024/day10.txt"));
        assert_eq!(part_1(&input), 737);
        assert_eq!(part_2(&input), 1619);
    }
}
