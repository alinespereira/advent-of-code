#![allow(unused)]
use crate::{AdventOfCodeSolver, Day};

impl AdventOfCodeSolver for Day<2022, 21> {
    fn solve_part_one(&mut self, input: String) {
        todo!();
    }

    fn solve_part_two(&mut self, input: String) {
        todo!();
    }
}

#[cfg(test)]
mod tests {
    #![allow(unused)]
    use std::fs;

    use super::*;

    #[test]
    fn it_solves_part_one() {
        let mut day: Day<2022, 21, _> = Day::new();
        let path = day.get_test_path();
        let input = fs::read_to_string(path).unwrap();
        day.solve_part_one(input);
        assert!(day.solution.part_one.is_some());
        assert_eq!(day.solution.part_one, None);
    }

    #[test]
    fn it_solves_part_two() {
        let mut day: Day<2022, 21, _> = Day::new();
        let path = day.get_test_path();
        let input = fs::read_to_string(path).unwrap();
        day.solve_part_two(input);
        assert!(day.solution.part_two.is_some());
        assert_eq!(day.solution.part_two, None);
    }
}
