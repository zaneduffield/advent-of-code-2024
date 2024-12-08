use fxhash::FxHashSet;

type Pos = (isize, isize);
type Dir = (i8, i8);

#[derive(Clone)]
pub struct Input {
    grid: Vec<Vec<Point>>,
    start: Pos,
    dir: Dir,
}

#[derive(Clone, Copy)]
pub enum Point {
    Empty,
    Full,
}

impl Input {
    fn get_point(&self, (row, col): Pos) -> Option<&Point> {
        self.grid
            .get(row as usize)
            .and_then(|row| row.get(col as usize))
    }
    fn get_point_mut(&mut self, (row, col): Pos) -> Option<&mut Point> {
        self.grid
            .get_mut(row as usize)
            .and_then(|row| row.get_mut(col as usize))
    }
}

fn parse_input(input: &str) -> Input {
    let mut start = (0, 0);
    let mut dir = None;
    let grid = input
        .lines()
        .enumerate()
        .map(|(row, line)| {
            line.bytes()
                .enumerate()
                .map(|(col, b)| match b {
                    b'#' => Point::Full,
                    b'.' => Point::Empty,
                    _ => {
                        start = (row as isize, col as isize);
                        dir = Some(match b {
                            b'^' => (-1, 0),
                            b'>' => (0, 1),
                            b'v' => (1, 0),
                            b'<' => (0, -1),
                            _ => panic!("unexpected byte in grid: {b:x}"),
                        });
                        Point::Empty
                    }
                })
                .collect()
        })
        .collect();

    Input {
        grid,
        start,
        dir: dir.expect("starting point should exist in grid"),
    }
}

pub fn input_generator(input: &str) -> Input {
    parse_input(input)
}

fn rotate((y, x): Dir) -> Dir {
    // (x + yi) * i = (-y + xi)
    (x, -y)
}

#[allow(unused)]
fn print(input: &Input, pos: Pos, dir: Dir) {
    eprintln!("=======================");
    let mut s = String::new();
    for (y, row) in input.grid.iter().enumerate() {
        for (x, col) in row.iter().enumerate() {
            if (y as isize, x as isize) == pos {
                s.push(match dir {
                    (-1, 0) => '^',
                    (0, 1) => '>',
                    (1, 0) => 'v',
                    (0, -1) => '<',
                    _ => unreachable!(),
                });
            } else {
                match col {
                    Point::Empty => s.push('.'),
                    Point::Full => s.push('#'),
                }
            }
        }
        s.push('\n');
    }
    eprintln!("{}", s);
    eprintln!("=======================");
}

pub fn part_1(input: &Input) -> u32 {
    let mut pos = input.start;
    let mut dir = input.dir;

    // A hashset would work, but this is much faster
    let mut visited_grid: Vec<Vec<bool>> = input
        .grid
        .iter()
        .map(|row| vec![false; row.len()])
        .collect();
    let mut visited_count = 0;

    loop {
        let p = &mut visited_grid[pos.0 as usize][pos.1 as usize];
        if !*p {
            *p = true;
            visited_count += 1;
        }

        loop {
            let next = (pos.0 + dir.0 as isize, pos.1 + dir.1 as isize);
            if let Some(p) = input.get_point(next) {
                if matches!(p, Point::Full) {
                    dir = rotate(dir);
                    continue;
                }
                pos = next;
                break;
            } else {
                return visited_count;
            }
        }
    }
}

type CycleSet = FxHashSet<(Pos, Dir)>;

fn is_cyclic(cycle: &mut CycleSet, input: &Input, mut pos: Pos, mut dir: Dir) -> bool {
    cycle.clear();

    loop {
        let next = (pos.0 + dir.0 as isize, pos.1 + dir.1 as isize);
        if let Some(p) = input.get_point(next) {
            if matches!(p, Point::Full) {
                dir = rotate(dir);
                if cycle.contains(&(pos, dir)) {
                    return true;
                }
                cycle.insert((pos, dir));
            } else {
                pos = next;
            }
        } else {
            return false;
        }
    }
}

pub fn part_2(input: &Input) -> u32 {
    let mut input = input.clone();

    let mut pos = input.start;
    let mut dir = input.dir;

    let mut new_barriers = FxHashSet::default();
    let mut path = FxHashSet::default();

    let mut cycle = FxHashSet::default();

    loop {
        path.insert(pos);

        let next = (pos.0 + dir.0 as isize, pos.1 + dir.1 as isize);
        if let Some(p) = input.get_point_mut(next) {
            if matches!(p, Point::Full) {
                dir = rotate(dir);
                continue;
            }

            // Pretend there was a barrier, would we then enter a cycle?
            // Make sure to not try placing a barrier on part of the path already completed.
            *p = Point::Full;
            if !path.contains(&next)
                && !new_barriers.contains(&next)
                && is_cyclic(&mut cycle, &input, pos, rotate(dir))
            {
                new_barriers.insert(next);
            }
            *input.get_point_mut(next).unwrap() = Point::Empty;

            pos = next;
        } else {
            return new_barriers.len() as u32;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test() {
        let input = input_generator(indoc! {
            "
            ....#.....
            .........#
            ..........
            ..#.......
            .......#..
            ..........
            .#..^.....
            ........#.
            #.........
            ......#...
            "
        });
        assert_eq!(part_1(&input), 41);
        assert_eq!(part_2(&input), 6);
    }

    #[test]
    fn test_my_input() {
        let input = input_generator(include_str!("../../input/2024/day6.txt"));
        assert_eq!(part_1(&input), 5129);
        assert_eq!(part_2(&input), 1888);
    }
}
