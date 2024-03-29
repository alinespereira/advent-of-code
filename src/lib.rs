pub mod downloader;
mod year_2022;

use std::fmt;
use std::fs;

pub trait AdventOfCodeSolver {
    fn solve_part_one(&mut self, input: String);
    fn solve_part_two(&mut self, input: String);
}

#[derive(Debug, Default)]
pub struct Solution<T>
where
    T: fmt::Debug + Default + fmt::Display,
{
    pub part_one: Option<T>,
    pub part_two: Option<T>,
}

#[derive(Debug, Default)]
pub struct Day<const Y: usize, const D: usize, T = usize>
where
    T: fmt::Debug + Default + fmt::Display,
{
    year_id: usize,
    day_id: usize,
    solution: Solution<T>,
}

impl<const Y: usize, const D: usize, T: Default> Day<Y, D, T>
where
    T: fmt::Debug + Default + fmt::Display,
{
    pub fn new() -> Self {
        return Self {
            year_id: Y,
            day_id: D,
            solution: Default::default(),
        };
    }

    pub fn get_input_path(&self) -> String {
        return format!("src/year_{}/day_{:02}/input.txt", self.year_id, self.day_id);
    }

    pub fn get_test_path(&self) -> String {
        return format!("src/year_{}/day_{:02}/test.txt", self.year_id, self.day_id);
    }

    pub fn load_input_data(&self) -> String {
        let path = format!("src/year_{}/day_{}/input.txt", self.year_id, self.day_id);
        return fs::read_to_string(path).unwrap();
    }
}

impl<const Y: usize, const D: usize, T> fmt::Display for Day<Y, D, T>
where
    T: fmt::Debug + Default + fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        let parts = vec![
            format!("[{:04}] Day {:02}", self.year_id, self.day_id),
            format!("\tPart 1: {:?}", self.solution.part_one),
            format!("\tPart 2: {:?}", self.solution.part_two),
        ];
        f.write_str(&parts.join("\n"))
    }
}
