use std::{fs, path::Path};

use advent_of_code::downloader::AoCDay;
use clap::Parser;

#[allow(unused)]
fn download_input(args: &AoCDay) -> Result<String, reqwest::Error> {
    let AoCDay { year, day } = args;
    let url = &format!("https://adventofcode.com/{year}/day/{day}/input")[..];
    return reqwest::blocking::get(url).and_then(|res| res.text());
}

#[derive(Debug)]
pub enum AoCDownloaderError {
    ReqwestError { error: reqwest::Error },
    IOError { error: std::io::Error },
}

impl From<reqwest::Error> for AoCDownloaderError {
    fn from(error: reqwest::Error) -> Self {
        return Self::ReqwestError { error };
    }
}

impl From<std::io::Error> for AoCDownloaderError {
    fn from(error: std::io::Error) -> Self {
        return Self::IOError { error };
    }
}

fn create_empty_file(base_path: &Path, file_name: &str) -> Result<(), AoCDownloaderError> {
    let file = Path::new(file_name);
    let file_path = base_path.join(file);

    if !file_path.exists() {
        let file_path = file_path.as_path();
        fs::write(file_path, "")?
    }

    return Ok(());
}

fn main() -> Result<(), AoCDownloaderError> {
    let _args @ AoCDay { year, day } = AoCDay::parse();
    let path_str = &format!("src/year_{}/day_{:02}", year, day)[..];
    let path = Path::new(path_str);

    if !path.exists() {
        fs::create_dir_all(path)?;
    }

    create_empty_file(path, "input.txt")?;
    create_empty_file(path, "test.txt")?;
    create_empty_file(path, "mod.rs")?;

    let mod_file = path.join(Path::new("mod.rs"));
    let mod_file = mod_file.as_path();

    let mut contents = String::new();
    contents.push_str(format!("#![allow(unused)]\n").as_str());
    contents.push_str(format!("use crate::{{AdventOfCodeSolver, Day}};\n").as_str());
    contents.push_str(format!("\n").as_str());
    contents.push_str(format!("impl AdventOfCodeSolver for Day<{year}, {day}> {{\n").as_str());
    contents.push_str(format!("    fn solve_part_one(&mut self, input: String) {{\n").as_str());
    contents.push_str(format!("        todo!();\n").as_str());
    contents.push_str(format!("    }}\n").as_str());
    contents.push_str(format!("\n").as_str());
    contents.push_str(format!("    fn solve_part_two(&mut self, input: String) {{\n").as_str());
    contents.push_str(format!("        todo!();\n").as_str());
    contents.push_str(format!("    }}\n").as_str());
    contents.push_str(format!("}}\n").as_str());
    contents.push_str(format!("\n").as_str());
    contents.push_str(format!("#[cfg(test)]\n").as_str());
    contents.push_str(format!("mod tests {{\n").as_str());
    contents.push_str(format!("    #![allow(unused)]\n").as_str());
    contents.push_str(format!("    use std::fs;\n").as_str());
    contents.push_str(format!("\n").as_str());
    contents.push_str(format!("    use super::*;\n").as_str());
    contents.push_str(format!("\n").as_str());
    contents.push_str(format!("    #[test]\n").as_str());
    contents.push_str(format!("    fn it_solves_part_one() {{\n").as_str());
    contents.push_str(format!("        let mut day: Day<{year}, {day}> = Day::new();\n").as_str());
    contents.push_str(format!("        todo!();\n").as_str());
    contents.push_str(format!("    }}\n").as_str());
    contents.push_str(format!("\n").as_str());
    contents.push_str(format!("    #[test]\n").as_str());
    contents.push_str(format!("    fn it_solves_part_two() {{\n").as_str());
    contents.push_str(format!("        let mut day: Day<{year}, {day}> = Day::new();\n").as_str());
    contents.push_str(format!("        todo!();\n").as_str());
    contents.push_str(format!("    }}\n").as_str());
    contents.push_str(format!("}}\n").as_str());

    fs::write(mod_file, contents)?;

    return Ok(());
}
