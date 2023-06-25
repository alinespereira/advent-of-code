use advent_of_code::{AdventOfCodeSolver, Day};
use std::fs;

fn main() {
    let mut aoc = Day::<2022, 1>::new();
    let path = aoc.get_input_path();
    let input: String = fs::read_to_string(path).unwrap();
    aoc.solve_part_one(input.clone());
    aoc.solve_part_two(input.clone());
    println!("{}", aoc);

    let mut aoc = Day::<2022, 2>::new();
    let path = aoc.get_input_path();
    let input: String = fs::read_to_string(path).unwrap();
    aoc.solve_part_one(input.clone());
    aoc.solve_part_two(input.clone());
    println!("{}", aoc);

    // let mut aoc = Day::<2022, 3>::new();
    // let path = aoc.get_input_path();
    // let input: String = fs::read_to_string(path).unwrap();
    // aoc.solve_part_one(input.clone());
    // aoc.solve_part_two(input.clone());
    // println!("{}", aoc);

    // let mut aoc = Day::<2022, 4>::new();
    // let path = aoc.get_input_path();
    // let input: String = fs::read_to_string(path).unwrap();
    // aoc.solve_part_one(input.clone());
    // aoc.solve_part_two(input.clone());
    // println!("{}", aoc);
}
