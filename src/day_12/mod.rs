use fxhash::FxHashSet;

pub type Input<'a> = Vec<&'a [u8]>;

pub fn input_generator(input: &str) -> Input {
    input.lines().map(|line| line.as_bytes()).collect()
}

fn flood(
    plant: u8,
    input: &Input,
    visited: &mut FxHashSet<(usize, usize)>,
    (y, x): (usize, usize),
) -> (u64, u64) {
    if visited.contains(&(y, x)) {
        return (0, 0);
    }
    visited.insert((y, x));

    let mut area = 0;
    let mut perimeter = 0;

    if Some(&plant) == input.get(y).and_then(|row| row.get(x)) {
        area += 1;

        for (dy, dx) in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
            let (y2, x2) = ((y as isize + dy) as usize, (x as isize + dx) as usize);
            match input.get(y2).and_then(|row| row.get(x2)) {
                Some(plant2) if *plant2 == plant => {
                    let (a2, p2) = flood(plant, input, visited, (y2, x2));
                    area += a2;
                    perimeter += p2;
                }
                _ => perimeter += 1,
            }
        }
    }

    (area, perimeter)
}

pub fn part_1(input: &Input) -> u64 {
    let mut visited = FxHashSet::default();

    let mut sum = 0;

    for (y, row) in input.iter().enumerate() {
        for (x, &col) in row.iter().enumerate() {
            let (area, perimiter) = flood(col, input, &mut visited, (y, x));

            sum += area * perimiter;
        }
    }

    sum
}

pub fn part_2(input: &Input) -> usize {
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
            AAAA
            BBCD
            BBCC
            EEEC
            "
        });
        assert_eq!(part_1(&input), 140);
        // assert_eq!(part_2(&input),);

        let input2 = input_generator(indoc! {
            "
            OOOOO
            OXOXO
            OOOOO
            OXOXO
            OOOOO
            "
        });
        assert_eq!(part_1(&input2), 772);

        let input3 = input_generator(indoc! {
            "
            RRRRIICCFF
            RRRRIICCCF
            VVRRRCCFFF
            VVRCCCJFFF
            VVVVCJJCFE
            VVIVCCJJEE
            VVIIICJJEE
            MIIIIIJJEE
            MIIISIJEEE
            MMMISSJEEE
            "
        });
        assert_eq!(part_1(&input3), 1930);
    }

    #[test]
    fn test_my_input() {
        let input = input_generator(include_str!("../../input/2024/day12.txt"));
        // assert_eq!(part_1(&input), );
        // assert_eq!(part_2(&input),);
    }
}
