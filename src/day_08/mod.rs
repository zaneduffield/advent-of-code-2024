use gcd::Gcd;

pub struct Input<'a> {
    data: Vec<&'a [u8]>,
}

pub fn input_generator(input: &str) -> Input {
    Input {
        data: input.lines().map(|s| s.as_bytes()).collect(),
    }
}

pub fn part_1(input: &Input) -> u32 {
    let mut antinode_map = input.data.iter().map(|r| r.to_vec()).collect::<Vec<_>>();

    let mut count = 0;

    for (y, row) in input.data.iter().enumerate() {
        for (x, &col) in row.iter().enumerate() {
            if matches!(col,
                b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9')
            {
                for (y2, row2) in input.data.iter().enumerate() {
                    for (x2, &col2) in row2.iter().enumerate() {
                        if col2 == col && (y2, x2) != (y, x) {
                            let (y3, x3) =
                                (2 * y2 as isize - y as isize, 2 * x2 as isize - x as isize);

                            if let Some(col) = antinode_map
                                .get_mut(y3 as usize)
                                .and_then(|row| row.get_mut(x3 as usize))
                            {
                                if *col != b'#' {
                                    *col = b'#';
                                    count += 1;
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    #[cfg(debug_assertions)]
    {
        antinode_map
            .iter()
            .for_each(|row| eprintln!("{}", std::str::from_utf8(row).unwrap()));
    }

    count
}

pub fn part_2(input: &Input) -> u32 {
    let mut antinode_map = input.data.iter().map(|r| r.to_vec()).collect::<Vec<_>>();

    let mut count = 0;

    for (y, row) in input.data.iter().enumerate() {
        for (x, &col) in row.iter().enumerate() {
            if matches!(col,
                b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9')
            {
                for (y2, row2) in input.data.iter().enumerate() {
                    for (x2, &col2) in row2.iter().enumerate() {
                        if col2 == col && (y2, x2) != (y, x) {
                            let (dy, dx) = (y2 as isize - y as isize, x2 as isize - x as isize);
                            let gcd = dy.unsigned_abs().gcd(dx.unsigned_abs()) as isize;
                            let (dy, dx) = (dy / gcd, dx / gcd);

                            let (mut y3, mut x3) = (y2 as isize, x2 as isize);
                            while let Some(col) = antinode_map
                                .get_mut(y3 as usize)
                                .and_then(|row| row.get_mut(x3 as usize))
                            {
                                if *col != b'#' {
                                    *col = b'#';
                                    count += 1;
                                }
                                y3 += dy;
                                x3 += dx;
                            }
                        }
                    }
                }
            }
        }
    }

    #[cfg(debug_assertions)]
    {
        antinode_map
            .iter()
            .for_each(|row| eprintln!("{}", std::str::from_utf8(row).unwrap()));
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test() {
        let input = input_generator(indoc! {
            "
            ............
            ........0...
            .....0......
            .......0....
            ....0.......
            ......A.....
            ............
            ............
            ........A...
            .........A..
            ............
            ............
            "

        });
        assert_eq!(part_1(&input), 14);
        assert_eq!(part_2(&input), 34);
    }

    #[test]
    fn test_my_input() {
        let input = input_generator(include_str!("../../input/2024/day8.txt"));
        assert_eq!(part_1(&input), 318);
        assert_eq!(part_2(&input), 1126);
    }
}
