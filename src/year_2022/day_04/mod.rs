#![allow(unused)]
use std::{ops::Range, str::FromStr};

use crate::{AdventOfCodeSolver, Day};

impl AdventOfCodeSolver for Day<2022, 4> {
    fn solve_part_one(&mut self, input: String) {
        let input = self.parse_input(input).unwrap();
        let containing_pairs: Vec<_> = input
            .iter()
            .filter(|(fst, snd)| fst.overlaps(snd) || snd.overlaps(fst))
            .collect();
        self.solution.part_one = Some(containing_pairs.len())
    }

    fn solve_part_two(&mut self, input: String) {
        let input = self.parse_input(input).unwrap();
        let overlapping_pairs: Vec<_> = input
            .iter()
            .filter(|(fst, snd)| fst.contains(snd))
            .collect();
        self.solution.part_two = Some(overlapping_pairs.len())
    }
}

impl Day<2022, 4> {
    fn parse_input(
        &self,
        input: String,
    ) -> Result<Vec<(Assignments, Assignments)>, AssignmentError> {
        input
            .split("\n")
            .flat_map(|line| line.split_once(","))
            .flat_map(|(fst, snd)| {
                let fst = Assignments::from_str(fst);
                let snd = Assignments::from_str(snd);
                fst.map(|fst| snd.map(|snd| (fst, snd)))
            })
            .collect()
    }
}

#[derive(Debug)]
struct Assignments(Range<usize>);

impl Assignments {
    pub fn overlaps(&self, other: &Self) -> bool {
        self.0.start <= other.0.start && self.0.end >= other.0.end
    }

    pub fn contains(&self, other: &Self) -> bool {
        self.0.start <= other.0.end && self.0.end >= other.0.start
    }
}

#[derive(Debug)]
enum AssignmentError {
    MalformedStr(String),
    InvalidNumber(String),
}

impl FromStr for Assignments {
    type Err = AssignmentError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((fst, snd)) = s.split_once("-") {
            let start = fst
                .parse()
                .map_err(|_| AssignmentError::MalformedStr(fst.to_string()))?;
            let end = snd
                .parse()
                .map_err(|_| AssignmentError::MalformedStr(snd.to_string()))?;
            Ok(Assignments(Range { start, end }))
        } else {
            Err(AssignmentError::MalformedStr(s.to_string()))
        }
    }
}

#[cfg(test)]
mod tests {
    #![allow(unused)]
    use std::fs;

    use super::*;

    #[test]
    fn it_solves_part_one() {
        let mut day: Day<2022, 4> = Day::new();
        let path = day.get_test_path();
        let input = fs::read_to_string(path).unwrap();
        day.solve_part_one(input);
        assert_eq!(day.solution.part_one, Some(2));
    }

    #[test]
    fn it_solves_part_two() {
        let mut day: Day<2022, 4> = Day::new();
        let path = day.get_test_path();
        let input = fs::read_to_string(path).unwrap();
        day.solve_part_two(input);
        assert_eq!(day.solution.part_two, Some(4));
    }
}
