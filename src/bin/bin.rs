use std::time::Instant;

use advent_of_code_2024::*;

#[cfg(feature = "io")]
macro_rules! input_str {
    ($d:expr) => {
        std::fs::read_to_string(concat!("input/2024/day", $d, ".txt")).unwrap()
    };
}

#[cfg(not(feature = "io"))]
macro_rules! input_str {
    ($d:expr) => {
        include_str!(concat!("../../input/2024/day", $d, ".txt"))
    };
}

macro_rules! run_parts {
    ($m:ident, $d:expr$(, $g:expr)?) => {
        let mut instant = Instant::now();
        let input = input_str!($d);
        $(
            let input = $g(&input);
            println!("day {}-parse ({:.1?})", $d, instant.elapsed());
            instant = Instant::now();
        )?
        let part1 = $m::part_1(&input);
        println!("day {}-1 ({:7.1?}): {}", $d, instant.elapsed(), part1);
        instant = Instant::now();
        let part2 = $m::part_2(&input);
        println!("day {}-2 ({:7.1?}): {}", $d, instant.elapsed(), part2);
        println!();
    };
}

macro_rules! run_day_with_generator {
    ($m:ident, $d:expr) => {
        run_parts!($m, $d, |i| $m::input_generator(i));
    };
}

#[allow(unused)]
macro_rules! run_day {
    ($m:ident, $d:expr) => {
        run_parts!($m, $d);
    };
}

pub fn main() {
    let instant = Instant::now();
    run_day_with_generator!(day_01, "1");
    run_day_with_generator!(day_02, "2");
    run_day_with_generator!(day_03, "3");
    run_day_with_generator!(day_04, "4");
    run_day_with_generator!(day_05, "5");
    run_day_with_generator!(day_06, "6");
    run_day_with_generator!(day_07, "7");
    run_day_with_generator!(day_08, "8");
    run_day_with_generator!(day_09, "9");
    run_day_with_generator!(day_10, "10");
    run_day_with_generator!(day_11, "11");
    run_day_with_generator!(day_12, "12");
    run_day_with_generator!(day_13, "13");

    println!("done in {:?}", instant.elapsed());
}
