use std::collections::BTreeMap;

const EMPTY_ID: i16 = -1;

#[derive(Clone)]
pub struct Input {
    data: Vec<i16>,
}

impl Input {
    fn checksum(&self) -> u64 {
        self.data
            .iter()
            .enumerate()
            .filter(|(_pos, id)| **id != EMPTY_ID)
            .map(|(pos, id)| pos as u64 * *id as u64)
            .sum()
    }
}

pub fn input_generator(input: &str) -> Input {
    let mut data = Vec::with_capacity(input.len() * 5);
    let mut input = input.trim_end().bytes();
    let mut id = 0i16;
    loop {
        if let Some(count) = input.next() {
            match count {
                b'0'..=b'9' => data.extend(std::iter::repeat_n(id, (count - b'0') as usize)),
                _ => panic!("unexpected byte: {count}"),
            }
        } else {
            break;
        }
        id = id.checked_add(1).expect("ID should not overflow");
        if let Some(count) = input.next() {
            match count {
                b'0'..=b'9' => data.extend(std::iter::repeat_n(EMPTY_ID, (count - b'0') as usize)),
                _ => panic!("unexpected byte: {count}"),
            }
        }
    }
    Input { data }
}

pub fn part_1(input: &Input) -> u64 {
    let mut input = input.clone();

    let mut left = 0;
    let mut right = input.data.len() - 1;
    while left < right {
        if input.data[right] != EMPTY_ID {
            if input.data[left] == EMPTY_ID {
                input.data.swap(left, right);
                left += 1;
                right -= 1;
            } else {
                left += 1;
            }
        } else {
            right -= 1;
        }
    }

    input.checksum()
}

pub fn part_2(input: &Input) -> u64 {
    let mut input = input.clone();

    let mut empties = BTreeMap::<usize, usize>::default();
    let mut pos = 0;
    let mut empty_size = 0;
    while pos < input.data.len() {
        if input.data[pos] == EMPTY_ID {
            empty_size += 1;
        } else {
            if empty_size > 0 {
                empties.insert(pos - empty_size, empty_size);
            }
            empty_size = 0;
        }
        pos += 1;
    }

    let mut right = input.data.len() - 1;
    while (0..input.data.len()).contains(&right) {
        let id = input.data[right];
        if id == EMPTY_ID {
            right = right.wrapping_sub(1);
            continue;
        }

        let mut group_size = 1;
        while input.data.get(right.wrapping_sub(1)) == Some(&id) {
            group_size += 1;
            right = right.wrapping_sub(1);
        }

        if let Some((&empty_pos, &empty_size)) = empties
            .iter()
            .take_while(|(idx, _size)| **idx < right)
            .find(|(_idx, size)| **size >= group_size)
        {
            if empty_pos < right {
                // found large enough spot, move the block
                input
                    .data
                    .copy_within(right..(right + group_size), empty_pos);
                // TODO memset?
                for idx in right..(right + group_size) {
                    input.data[idx] = EMPTY_ID;
                }

                // we may have just created a new empty position, by splitting the old one
                empties.remove(&empty_pos);
                let excess_space = empty_size - group_size;
                if excess_space > 0 {
                    empties.insert(empty_pos + group_size, excess_space);
                }
            }
        }

        right = right.wrapping_sub(1);
    }

    input.checksum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = input_generator("2333133121414131402");
        assert_eq!(part_1(&input), 1928);
        assert_eq!(part_2(&input), 2858);
    }

    #[test]
    fn test_my_input() {
        let input = input_generator(include_str!("../../input/2024/day9.txt"));
        assert_eq!(part_1(&input), 6337921897505);
        assert_eq!(part_2(&input), 6362722604045);
    }
}
