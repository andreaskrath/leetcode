use glob::glob;
use std::{
    fs::{read_to_string, OpenOptions},
    io::Write,
    path::PathBuf,
};

#[derive(Debug)]
pub struct Solution {
    pub number: usize,
    pub name: String,
    pub link: String,
    pub lang: String,
    pub path: String,
}

impl Solution {
    fn to_table_string(&self) -> String {
        format!(
            "|{}|[{}]({})|[Rust]({})|\n",
            self.number, self.name, self.link, self.path
        )
    }
}

pub fn index_solutions() -> Vec<Solution> {
    let mut solutions = Vec::new();
    for entry in glob("../solutions/**/src/main.rs").expect("failed to read glob pattern") {
        match entry {
            Ok(path) => {
                let solution = index_single_solition(path);
                solutions.push(solution);
            }
            Err(e) => panic!("{}", e),
        }
    }

    solutions
}

fn index_single_solition(path: PathBuf) -> Solution {
    let file = read_to_string(path.clone()).expect("failed to read file");
    let lines: Vec<&str> = file.split('\n').collect();

    let line1: Vec<&str> = lines[0].trim_start_matches("//").split('.').collect();
    Solution {
        number: line1[0]
            .trim()
            .parse::<usize>()
            .expect("failed to parse number"),
        name: line1[1].trim().to_string(),
        link: lines[1].trim_start_matches("//").trim().to_string(),
        lang: String::from("Rust"),
        path: path
            .to_str()
            .expect("failed to convert PathBuf to &str")
            .strip_prefix("../")
            .expect("failed to remove prefix")
            .to_string(),
    }
}

pub fn update_readme(solutions: Vec<Solution>) -> usize {
    let start_lines = read_to_string("../README.md")
        .expect("failed to read README.md")
        .split('\n')
        .count();

    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("../README.md")
        .expect("failed to edit README.md");

    let table_header = "|Nr.|Name|Solution|\n";
    let table_orientation = "|:-:|:-:|:-:|\n";
    _ = file
        .write(table_header.as_bytes())
        .expect("failed to write table header");
    _ = file
        .write(table_orientation.as_bytes())
        .expect("failed to write table orientation");

    for solution in solutions {
        _ = file.write(solution.to_table_string().as_bytes())
    }

    let end_lines = read_to_string("../README.md")
        .expect("failed to read README.md")
        .split('\n')
        .count();
    end_lines - start_lines
}
