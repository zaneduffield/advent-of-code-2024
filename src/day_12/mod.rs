use fxhash::FxHashSet;

pub type Input<'a> = Vec<&'a [u8]>;

pub fn input_generator(input: &str) -> Input {
    input.lines().map(|line| line.as_bytes()).collect()
}

fn flood_area_perimeter(
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
                    let (a2, p2) = flood_area_perimeter(plant, input, visited, (y2, x2));
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
            let (area, perimiter) = flood_area_perimeter(col, input, &mut visited, (y, x));

            sum += area * perimiter;
        }
    }

    sum
}

fn rotate_clockwise((y, x): (isize, isize)) -> (isize, isize) {
    (x, -y)
}
fn rotate_anticlockwise((y, x): (isize, isize)) -> (isize, isize) {
    (-x, y)
}

fn flood_area_sides(
    plant: u8,
    input: &Input,
    visited: &mut FxHashSet<(usize, usize)>,
    visited_edge: &mut FxHashSet<((isize, isize), (usize, usize))>,
    (y, x): (usize, usize),
) -> (u64, u64) {
    if visited.contains(&(y, x)) {
        return (0, 0);
    }
    visited.insert((y, x));

    let mut area = 0;
    let mut sides = 0;

    if Some(&plant) == input.get(y).and_then(|row| row.get(x)) {
        area += 1;

        // explore the edges first
        for (dy, dx) in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
            if visited_edge.contains(&((dy, dx), (y, x))) {
                continue;
            }
            visited_edge.insert(((dy, dx), (y, x)));

            let (y2, x2) = ((y as isize + dy) as usize, (x as isize + dx) as usize);
            match input.get(y2).and_then(|row| row.get(x2)) {
                Some(plant2) if *plant2 == plant => {}
                _ => {
                    sides += 1;
                    // now explore the whole side (without recursing)
                    for (dy2, dx2) in [rotate_clockwise((dy, dx)), rotate_anticlockwise((dy, dx))] {
                        let (mut y, mut x) = (y as isize + dy2, x as isize + dx2);
                        while input.get(y as usize).and_then(|row| row.get(x as usize))
                            == Some(&plant)
                            && input
                                .get((y + dy) as usize)
                                .and_then(|row| row.get((x + dx) as usize))
                                != Some(&plant)
                        {
                            visited_edge.insert(((dy, dx), (y as usize, x as usize)));
                            (y, x) = (y + dy2, x + dx2);
                        }
                    }
                }
            }
        }

        // then explore the interior
        for (dy, dx) in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
            let (y2, x2) = ((y as isize + dy) as usize, (x as isize + dx) as usize);
            match input.get(y2).and_then(|row| row.get(x2)) {
                Some(plant2) if *plant2 == plant => {
                    let (a2, p2) = flood_area_sides(plant, input, visited, visited_edge, (y2, x2));
                    area += a2;
                    sides += p2;
                }
                _ => {}
            }
        }
    }

    (area, sides)
}

pub fn part_2(input: &Input) -> u64 {
    let mut visited = FxHashSet::default();
    let mut visited_edge = FxHashSet::default();

    let mut sum = 0;

    for (y, row) in input.iter().enumerate() {
        for (x, &col) in row.iter().enumerate() {
            let (area, sides) =
                flood_area_sides(col, input, &mut visited, &mut visited_edge, (y, x));

            sum += area * sides;
        }
    }

    sum
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
        assert_eq!(part_2(&input), 80);

        let input = input_generator(indoc! {
            "
            OOOOO
            OXOXO
            OOOOO
            OXOXO
            OOOOO
            "
        });
        assert_eq!(part_1(&input), 772);

        let input = input_generator(indoc! {
            "
            EEEEE
            EXXXX
            EEEEE
            EXXXX
            EEEEE
            "
        });
        assert_eq!(part_2(&input), 236);

        let input = input_generator(indoc! {
            "
            AAAAAA
            AAABBA
            AAABBA
            ABBAAA
            ABBAAA
            AAAAAA
            "
        });
        assert_eq!(part_2(&input), 368);

        let input = input_generator(indoc! {
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
        assert_eq!(part_1(&input), 1930);
        assert_eq!(part_2(&input), 1206);
    }

    #[test]
    fn test_my_input() {
        let input = input_generator(include_str!("../../input/2024/day12.txt"));
        assert_eq!(part_1(&input), 1485656);
        // assert_eq!(part_2(&input),);
    }
}
