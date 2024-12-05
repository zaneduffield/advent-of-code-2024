pub struct Input {
    before_map: [Vec<u8>; 256],
    pages: Vec<Vec<u8>>,
}

fn parse_input(input: &str) -> Input {
    let mut before_map = std::array::from_fn(|_| Vec::with_capacity(8));
    let mut lines = input.lines();
    for line in lines.by_ref() {
        if line.is_empty() {
            break;
        }
        let (left, right) = line
            .split_once("|")
            .expect("lines should be delimited by |");
        let (left, right): (u8, u8) = (
            left.parse().expect("value should parse into u8"),
            right.parse().expect("value should parse into u8"),
        );

        before_map[left as usize].push(right);
    }

    let pages = lines
        .map(|line| {
            line.split(',')
                .map(|n| n.parse::<u8>().expect("value should parse into u8"))
                .collect()
        })
        .collect();

    Input { before_map, pages }
}

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> Input {
    parse_input(input)
}

fn is_page_ordered(input: &Input, page: &[u8]) -> bool {
    let mut seen = [false; 256];

    for num in page.iter() {
        seen[*num as usize] = true;
        for requirement in &input.before_map[*num as usize] {
            if seen[*requirement as usize] {
                return false;
            }
        }
    }

    true
}

#[allow(clippy::mut_range_bound)]
fn reorder_page(input: &Input, page: &mut [u8]) {
    let mut seen = [false; 256];

    let mut i = 0;
    'outer: while i < page.len() {
        let num = page[i];
        seen[num as usize] = true;
        for requirement in &input.before_map[num as usize] {
            for j in 0..i {
                if page[j] == *requirement {
                    page.swap(i, j);
                    i = j;
                    continue 'outer;
                }
            }
        }

        i += 1;
    }
}

#[aoc(day5, part1)]
pub fn part_1(input: &Input) -> u32 {
    input
        .pages
        .iter()
        .filter(|page| is_page_ordered(input, page))
        .map(|page| page[page.len() / 2] as u32)
        .sum()
}

#[aoc(day5, part2)]
pub fn part_2(input: &Input) -> u32 {
    input
        .pages
        .clone()
        .into_iter()
        .filter(|page| !is_page_ordered(input, page))
        .map(|mut page| {
            reorder_page(input, &mut page);
            page
        })
        .map(|page| page[page.len() / 2] as u32)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test() {
        let input = input_generator(indoc! {
            "
            47|53
            97|13
            97|61
            97|47
            75|29
            61|13
            75|53
            29|13
            97|29
            53|29
            61|53
            97|53
            61|29
            47|13
            75|47
            97|75
            47|61
            75|61
            47|29
            75|13
            53|13

            75,47,61,53,29
            97,61,53,29,13
            75,29,13
            75,97,47,61,53
            61,13,29
            97,13,75,29,47
            "
        });
        assert_eq!(part_1(&input), 143);
        assert_eq!(part_2(&input), 123);
    }

    #[test]
    fn test_my_input() {
        let input = input_generator(include_str!("../../input/2024/day5.txt"));
        assert_eq!(part_1(&input), 4637);
        assert_eq!(part_2(&input), 6370);
    }
}
