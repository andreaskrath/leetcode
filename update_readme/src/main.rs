mod solution;

use solution::{index_solutions, update_readme};

fn main() {
    let mut solutions = index_solutions();
    solutions.sort_by(|a, b| a.number.partial_cmp(&b.number).unwrap());
    let new_lines = update_readme(solutions);

    let solution_format = if new_lines == 1 {
        "solution"
    } else {
        "solutions"
    };
    println!(
        "{} new {} was added to the README.md file.",
        new_lines, solution_format
    );
}
