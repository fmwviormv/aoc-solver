use super::super::{ParseError, Problem, SolveError, Solver};

pub const PROBLEM: Problem = Problem {
    year: 2015,
    day: 2,
    title: "I Was Told There Would Be No Math",
    parts: 2,
    parse: |input| Ok(Box::new(Day2::new(input)?)),
};

struct Day2 {
    paper: u128,
    ribbon: u128,
}

impl Day2 {
    fn new<'a>(input: &'a str) -> Result<Self, ParseError<'a>> {
        let mut res = Day2 {
            paper: 0,
            ribbon: 0,
        };
        for (i, line) in input.lines().enumerate() {
            const SPLIT: &str = "x";
            let mut pos = 0usize;
            let mut values = [0u32; 3];
            let mut parts = line.splitn(values.len(), SPLIT);
            for value in &mut values {
                pos += 1;
                if let Some(part) = parts.next() {
                    *value = match part.parse() {
                        Ok(value) => value,
                        Err(err) => {
                            return Err(ParseError {
                                msg: "",
                                line: i + 1,
                                pos,
                                source: Some(Box::new(err)),
                                arg: part,
                            })
                        }
                    };
                    pos += part.chars().count();
                } else {
                    return Err(ParseError {
                        msg: "Expected data",
                        line: i + 1,
                        pos,
                        source: None,
                        arg: "",
                    });
                }
            }
            values.sort_unstable();
            let x = values[0] as u128;
            let y = values[1] as u128;
            let z = values[2] as u128;
            res.paper += 3 * x * y + 2 * (x + y) * z;
            res.ribbon += 2 * (x + y) + x * y * z;
        }
        Ok(res)
    }

    fn part1(&mut self) -> String {
        self.paper.to_string()
    }

    fn part2(&mut self) -> String {
        self.ribbon.to_string()
    }
}

impl Solver for Day2 {
    fn solve(&mut self, part: u8) -> Result<String, SolveError> {
        match part {
            1 => Ok(self.part1()),
            2 => Ok(self.part2()),
            _ => Err(SolveError::PartNotFound(part)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(part: u8, input: &str, expect: &str) {
        let mut solver = Day2::new(input).unwrap();
        assert_eq!(solver.solve(part), Ok(expect.into()));
    }

    #[test]
    fn bad() {
        for input in ["2", "2x3", "2x3xZ", "2x3x4x5"] {
            if let Ok(_) = Day2::new(input) {
                panic!();
            }
        }
    }

    #[test]
    fn part1() {
        test(1, "2x3x4", "58");
        test(1, "1x1x10", "43");
    }

    #[test]
    fn part2() {
        test(2, "2x3x4", "34");
        test(2, "1x1x10", "14");
    }
}
