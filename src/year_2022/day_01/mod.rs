use crate::{AdventOfCodeSolver, Day};

impl AdventOfCodeSolver for Day<2022, 1> {
    fn solve_part_one(&mut self, input: String) {
        let rows: Vec<Option<usize>> = input.split('\n').map(|row| row.parse().ok()).collect();
        let rows: Vec<&[Option<usize>]> = rows.split(|row| row.is_none()).collect();
        let mut rows: Vec<usize> = rows.iter().map(|&row| row.iter().flatten().sum()).collect();
        rows.sort_by(|a, b| b.cmp(a));
        self.solution.part_one = Some(rows[0]);
    }

    fn solve_part_two(&mut self, input: String) {
        let rows: Vec<Option<usize>> = input.split('\n').map(|row| row.parse().ok()).collect();
        let rows: Vec<&[Option<usize>]> = rows.split(|row| row.is_none()).collect();
        let mut rows: Vec<usize> = rows.iter().map(|&row| row.iter().flatten().sum()).collect();
        rows.sort_by(|a, b| b.cmp(a));
        self.solution.part_two = Some(rows[0..=2].iter().sum());
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn it_solves_part_one() {
        let mut day: Day<2022, 1> = Day::new();
        let path = day.get_test_path();
        dbg!(path.clone());
        let input: String = fs::read_to_string(path).unwrap();
        day.solve_part_one(input);
        assert_eq!(day.solution.part_one, Some(24000));
    }

    #[test]
    fn it_solves_part_two() {
        let mut day: Day<2022, 1> = Day::new();
        let path = day.get_test_path();
        dbg!(path.clone());
        let input: String = fs::read_to_string(path).unwrap();
        day.solve_part_two(input);
        assert_eq!(day.solution.part_two, Some(45000));
    }
}
