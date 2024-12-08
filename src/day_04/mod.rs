pub struct Input {
    data: Vec<Vec<u8>>,
}

fn parse_input(input: &str) -> nom::IResult<&str, Input> {
    Ok((
        "",
        Input {
            data: input.split('\n').map(|s| s.as_bytes().to_vec()).collect(),
        },
    ))
}

pub fn input_generator(input: &str) -> Input {
    let (remaining, result) = parse_input(input).expect("failed to parse input");
    assert!(remaining.trim().is_empty(), "failed to parse entire input");
    result
}

pub fn part_1(input: &Input) -> u32 {
    let mut solutions = 0;
    for (y, row) in input.data.iter().enumerate() {
        for (x, col) in row.iter().enumerate() {
            if *col != b'X' {
                continue;
            }

            for (dx, dy) in [
                (-1, 0),
                (-1, -1),
                (0, -1),
                (1, -1),
                (1, 0),
                (1, 1),
                (0, 1),
                (-1, 1),
            ] {
                const TARGET: &[u8] = b"XMAS";
                let mut pos = 1usize;

                let mut x = x as isize;
                let mut y = y as isize;

                loop {
                    x += dx;
                    y += dy;

                    if y.try_into().ok().zip(x.try_into().ok()).and_then(
                        |(y, x): (usize, usize)| input.data.get(y).and_then(|row| row.get(x)),
                    ) != TARGET.get(pos)
                    {
                        break;
                    }
                    pos += 1;
                    if pos >= TARGET.len() {
                        solutions += 1;
                        break;
                    }
                }
            }
        }
    }

    solutions
}

pub fn part_2(input: &Input) -> u32 {
    let mut solutions = 0;
    for (y, row) in input.data.iter().enumerate() {
        'row: for (x, col) in row.iter().enumerate() {
            if *col != b'A' {
                continue;
            }

            let mut corners =
                [(-1, -1), (1, -1), (1, 1), (-1, 1)].map(|(dx, dy): (isize, isize)| {
                    let x = x as isize + dx;
                    let y = y as isize + dy;

                    y.try_into()
                        .ok()
                        .zip(x.try_into().ok())
                        .and_then(|(y, x): (usize, usize)| {
                            input.data.get(y).and_then(|row| row.get(x).cloned())
                        })
                        .unwrap_or(b'0')
                });

            for _ in 0..4 {
                corners.rotate_left(1);
                if corners == *b"MSSM" {
                    solutions += 1;
                    continue 'row;
                }
            }
        }
    }

    solutions
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test() {
        let input = input_generator(indoc! {
            "
            MMMSXXMASM
            MSAMXMSMSA
            AMXSXMAAMM
            MSAMASMSMX
            XMASAMXAMM
            XXAMMXXAMA
            SMSMSASXSS
            SAXAMASAAA
            MAMMMXMMMM
            MXMXAXMASX
            "
        });
        assert_eq!(part_1(&input), 18);
        assert_eq!(part_2(&input), 9);
    }

    #[test]
    fn test_my_input() {
        let input = input_generator(include_str!("../../input/2024/day4.txt"));
        assert_eq!(part_1(&input), 2532);
        assert_eq!(part_2(&input), 1941);
    }
}
