use super::super::{ParseError, Problem, SolveError, Solver};

pub const PROBLEM: Problem = Problem {
    year: 2015,
    day: 1,
    title: "Not Quite Lisp",
    parts: 2,
    parse: |input| Ok(Box::new(Day1::new(input)?)),
};

struct Day1 {
    open: usize,
    close: usize,
    pos: usize,
}

impl Day1 {
    fn new<'a>(input: &'a str) -> Result<Self, ParseError<'a>> {
        let mut res = Day1 {
            open: 0,
            close: 0,
            pos: 0,
        };
        for (i, ch) in input.chars().enumerate() {
            match ch {
                '(' => res.open += 1,
                ')' => res.close += 1,
                _ => {
                    return Err(ParseError {
                        msg: "Invalid character",
                        line: 1,
                        pos: i + 1,
                        source: None,
                        arg: &input[i..=i],
                    });
                }
            }
            if res.pos == 0 && res.open + 1 == res.close {
                res.pos = i + 1;
            }
        }
        Ok(res)
    }

    fn part1(&mut self) -> String {
        if self.open >= self.close {
            (self.open - self.close).to_string()
        } else {
            format!("-{}", self.close - self.open)
        }
    }

    fn part2(&mut self) -> String {
        self.pos.to_string()
    }
}

impl Solver for Day1 {
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
        let mut solver = Day1::new(input).unwrap();
        assert_eq!(solver.solve(part), Ok(expect.into()));
    }

    #[test]
    fn bad() {
        for input in [")\n"] {
            if let Ok(_) = Day1::new(input) {
                panic!();
            }
        }
    }

    #[test]
    fn part1() {
        test(1, "(())", "0");
        test(1, "()()", "0");
        test(1, "(((", "3");
        test(1, "(()(()(", "3");
        test(1, "))(((((", "3");
        test(1, "())", "-1");
        test(1, "))(", "-1");
        test(1, ")))", "-3");
        test(1, ")())())", "-3");
    }

    #[test]
    fn part2() {
        test(2, ")", "1");
        test(2, "()())", "5");
    }
}
