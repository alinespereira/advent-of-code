#![allow(unused)]
use std::collections::{hash_set, HashSet};

use crate::{AdventOfCodeSolver, Day};

impl AdventOfCodeSolver for Day<2022, 3> {
    fn solve_part_one(&mut self, input: String) {
        let input = self.parse_input(input);
        let solution = input
            .iter()
            .map(|rs| {
                let common = rs.common_items();
                let common = common.first();
                common.map(|item| (*item).score()).unwrap()
            })
            .sum();
        self.solution.part_one = Some(solution);
    }

    fn solve_part_two(&mut self, input: String) {
        let input = self.parse_input(input);
        let solution: Vec<_> = input
            .chunks(3)
            .map(|rs| {
                let item = RuckSack::compare(&rs[0], &rs[1], &rs[2]);
                item.iter().map(|&i| i.score()).collect::<Vec<_>>()
            })
            .collect();
        let solution: usize = dbg!(solution.iter().flat_map(|it| it.first()).sum());
        self.solution.part_two = Some(solution);
    }
}

impl Day<2022, 3> {
    fn parse_input(&self, input: String) -> Vec<RuckSack> {
        input
            .split('\n')
            .map(|line| -> RuckSack {
                let line: Vec<CarriedItem> = line.chars().map(|ch| CarriedItem(ch)).collect();
                let (fst, snd) = line.split_at(line.len() / 2);
                RuckSack {
                    first_compartiment: Compartiment {
                        items: HashSet::from_iter(fst.iter().cloned()),
                    },
                    second_compartiment: Compartiment {
                        items: HashSet::from_iter(snd.iter().cloned()),
                    },
                }
            })
            .collect()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
struct CarriedItem(char);
impl CarriedItem {
    fn score(self) -> usize {
        let lowercase: Vec<char> = ('a'..='z').collect();
        let uppercase: Vec<char> = ('A'..='Z').collect();
        let mut letters: Vec<char> = Vec::new();
        letters.extend(lowercase);
        letters.extend(uppercase);
        let ch = self.0;
        let scores: Vec<usize> = letters
            .iter()
            .zip(1..)
            .filter(|&(&c, _)| c == ch)
            .map(|(_, score)| score)
            .collect();
        return *scores.first().unwrap_or(&0);
    }
}

#[derive(Debug)]
struct Compartiment {
    items: HashSet<CarriedItem>,
}

impl Compartiment {
    pub fn common_items(&self, other: &Self) -> Vec<CarriedItem> {
        self.items.intersection(&other.items).cloned().collect()
    }
}

#[derive(Debug)]
struct RuckSack {
    pub first_compartiment: Compartiment,
    pub second_compartiment: Compartiment,
}

impl RuckSack {
    pub fn common_items(&self) -> Vec<CarriedItem> {
        self.first_compartiment
            .common_items(&self.second_compartiment)
    }

    fn all_items(&self) -> Vec<CarriedItem> {
        self.first_compartiment
            .items
            .union(&self.second_compartiment.items)
            .cloned()
            .collect()
    }

    pub fn compare(r: &RuckSack, s: &RuckSack, t: &RuckSack) -> Vec<CarriedItem> {
        let r: HashSet<CarriedItem> = HashSet::from_iter(r.all_items().iter().cloned());
        let s: HashSet<CarriedItem> = HashSet::from_iter(s.all_items().iter().cloned());
        let t: HashSet<CarriedItem> = HashSet::from_iter(t.all_items().iter().cloned());
        r.intersection(&s.intersection(&t).cloned().collect())
            .cloned()
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn it_solves_part_one() {
        let mut day: Day<2022, 3> = Day::new();
        let path = day.get_test_path();
        let input = fs::read_to_string(path).unwrap();
        day.solve_part_one(input);
        assert_eq!(day.solution.part_one, Some(157));
    }

    #[test]
    fn it_solves_part_two() {
        let mut day: Day<2022, 3> = Day::new();
        let path = day.get_test_path();
        let input = fs::read_to_string(path).unwrap();
        day.solve_part_two(input);
        assert_eq!(day.solution.part_two, Some(70));
    }
}
