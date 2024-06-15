use std::io::{stdin, Read};

use aoc_solver::PROBLEMS;

fn main() {
    let mut args = std::env::args();
    args.next().unwrap();
    let year: u16 = args.next().unwrap().parse().unwrap();
    let day: u8 = args.next().unwrap().parse().unwrap();
    for problem in PROBLEMS {
        if problem.year() == year && problem.day() == day {
            let input = match args.next() {
                Some(arg) => arg,
                None => {
                    let mut input = String::new();
                    stdin().read_to_string(&mut input).unwrap();
                    input
                }
            };
            let mut solver = problem.parse(&input).unwrap();
            for part in 1..=problem.parts() {
                let output = solver.solve(part).unwrap();
                println!("{}", output);
            }
            return;
        }
    }
    panic!("Not implemented");
}
