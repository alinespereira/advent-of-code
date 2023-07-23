use advent_of_code::{AdventOfCodeSolver, Day};

#[macro_export]
macro_rules! run_aoc {
    ($year:literal/$day:literal) => {
        let mut aoc = Day::<$year, $day>::new();
        let path = aoc.get_input_path();
        let input: String = std::fs::read_to_string(path).unwrap();
        aoc.solve_part_one(input.clone());
        aoc.solve_part_two(input.clone());
        println!("{}", aoc);
    };
    (<$t:ty>$year:literal/$day:literal) => {
        let mut aoc = Day::<$year, $day, $t>::new();
        let path = aoc.get_input_path();
        let input: String = std::fs::read_to_string(path).unwrap();
        aoc.solve_part_one(input.clone());
        aoc.solve_part_two(input.clone());
        println!("{}", aoc);
    };
}

fn main() {
    run_aoc!(2022 / 1);
    run_aoc!(2022 / 2);
    run_aoc!(2022 / 3);
    run_aoc!(2022 / 4);
    run_aoc!(<&str>2022 / 5);
    // run_aoc!(2022 / 6);
    // run_aoc!(2022 / 7);
    // run_aoc!(2022 / 8);
    // run_aoc!(2022 / 9);
    // run_aoc!(2022 / 10);
    // run_aoc!(2022 / 11);
    // run_aoc!(2022 / 12);
    // run_aoc!(2022 / 13);
    // run_aoc!(2022 / 14);
    // run_aoc!(2022 / 15);
    // run_aoc!(2022 / 16);
    // run_aoc!(2022 / 17);
    // run_aoc!(2022 / 18);
    // run_aoc!(2022 / 19);
    // run_aoc!(2022 / 20);
    // run_aoc!(2022 / 21);
    // run_aoc!(2022 / 22);
    // run_aoc!(2022 / 23);
    // run_aoc!(2022 / 24);
    // run_aoc!(2022 / 25);
}
