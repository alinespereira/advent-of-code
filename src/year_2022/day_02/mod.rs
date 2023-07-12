use std::cmp::Ordering;

use strum::{EnumIter, IntoEnumIterator};

use crate::{AdventOfCodeSolver, Day};

impl AdventOfCodeSolver for Day<2022, 2> {
    fn solve_part_one(&mut self, input: String) {
        let input: Vec<(Shape, Shape)> = self.parse_input(input);
        let scores: Vec<usize> = input
            .into_iter()
            .map(|(opponent, player)| self.score_match(opponent, player))
            .collect();
        let solution: usize = scores.iter().sum();
        self.solution.part_one = Some(solution)
    }

    fn solve_part_two(&mut self, input: String) {
        let input: Vec<(Shape, Status)> = self.parse_input(input);
        let scores: Vec<usize> = input
            .into_iter()
            .map(|(opponent, status)| (opponent, self.pick_shape(opponent, status).unwrap()))
            .map(|(opponent, player)| self.score_match(opponent, player))
            .collect();
        let solution: usize = scores.iter().sum();
        self.solution.part_two = Some(solution)
    }
}

#[derive(Debug, PartialEq, Clone, Copy, EnumIter)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    pub fn score(self) -> usize {
        match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Status {
    Win,
    Draw,
    Lose,
}

impl Status {
    pub fn score(self) -> usize {
        match self {
            Status::Win => 6,
            Status::Draw => 3,
            Status::Lose => 0,
        }
    }
}

impl From<char> for Shape {
    fn from(ch: char) -> Self {
        match ch {
            'A' | 'X' => Shape::Rock,
            'B' | 'Y' => Shape::Paper,
            'C' | 'Z' => Shape::Scissors,
            _ => panic!("Invalid shape!"),
        }
    }
}

impl From<char> for Status {
    fn from(ch: char) -> Self {
        match ch {
            'X' => Status::Lose,
            'Y' => Status::Draw,
            'Z' => Status::Win,
            _ => panic!("Invalid Shape!"),
        }
    }
}

impl From<Shape> for usize {
    fn from(shape: Shape) -> Self {
        match shape {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }
}

impl PartialOrd for Shape {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Shape::Rock, Shape::Rock) => Some(Ordering::Equal),
            (Shape::Paper, Shape::Paper) => Some(Ordering::Equal),
            (Shape::Scissors, Shape::Scissors) => Some(Ordering::Equal),

            (Shape::Rock, Shape::Scissors) => Some(Ordering::Greater),
            (Shape::Scissors, Shape::Paper) => Some(Ordering::Greater),
            (Shape::Paper, Shape::Rock) => Some(Ordering::Greater),

            (Shape::Scissors, Shape::Rock) => Some(Ordering::Less),
            (Shape::Paper, Shape::Scissors) => Some(Ordering::Less),
            (Shape::Rock, Shape::Paper) => Some(Ordering::Less),
        }
    }
}

impl Day<2022, 2> {
    fn parse_input<T: From<char>, U: From<char>>(&self, input: String) -> Vec<(T, U)> {
        input
            .split('\n')
            .map(|line| {
                let line: Vec<char> = line.chars().filter(|&ch| ch != ' ').take(2).collect();
                let line: &[char] = line.as_slice();
                (line[0].into(), line[1].into())
            })
            .collect()
    }

    fn battle(&self, opponent_shape: Shape, player_shape: Shape) -> Option<Status> {
        player_shape
            .partial_cmp(&opponent_shape)
            .map(|cmp| match cmp {
                Ordering::Less => Status::Lose,
                Ordering::Equal => Status::Draw,
                Ordering::Greater => Status::Win,
            })
    }

    fn score_match(&self, opponent_shape: Shape, player_shape: Shape) -> usize {
        return self
            .battle(opponent_shape, player_shape)
            .map(|status| status.score() + player_shape.score())
            .unwrap_or_default();
    }

    fn pick_shape(&self, opponent_shape: Shape, match_result: Status) -> Option<Shape> {
        for player_shape in Shape::iter() {
            if let Some(status) = self.battle(opponent_shape, player_shape) {
                if status == match_result {
                    return Some(player_shape);
                }
            }
        }
        return None;
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn it_solves_part_one() {
        let mut day: Day<2022, 2> = Day::new();
        let path = day.get_test_path();
        let input = fs::read_to_string(path).unwrap();
        day.solve_part_one(input);
        assert_eq!(day.solution.part_one, Some(15));
    }

    #[test]
    fn it_solves_part_two() {
        let mut day: Day<2022, 2> = Day::new();
        let path = day.get_test_path();
        let input = fs::read_to_string(path).unwrap();
        day.solve_part_two(input);
        assert_eq!(day.solution.part_two, Some(12));
    }
}
