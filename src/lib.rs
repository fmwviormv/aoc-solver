use std::{error, fmt};

pub struct Problem {
    year: u16,
    day: u8,
    title: &'static str,
    parts: u8,
    parse: for<'a> fn(&'a str) -> Result<Box<dyn Solver>, ParseError<'a>>,
}

impl Problem {
    pub fn year(&self) -> u16 {
        self.year
    }
    pub fn day(&self) -> u8 {
        self.day
    }
    pub fn title(&self) -> &str {
        self.title
    }
    pub fn parts(&self) -> u8 {
        self.parts
    }
    pub fn parse<'a>(&self, input: &'a str) -> Result<Box<dyn Solver>, ParseError<'a>> {
        (self.parse)(input)
    }
    pub fn solve_all<'a>(&self, input: &'a str) -> Result<Vec<String>, ParseError<'a>> {
        let mut res = Vec::<String>::new();
        let mut solver = self.parse(input)?;
        for part in 1..=self.parts {
            res.push(solver.solve(part).unwrap());
        }
        Ok(res)
    }
}

impl fmt::Display for Problem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Day {}: {}", self.day, self.title)
    }
}

#[derive(Debug)]
pub struct ParseError<'a> {
    msg: &'static str,
    line: usize,
    pos: usize,
    source: Option<Box<dyn error::Error + 'static>>,
    arg: &'a str,
}

impl<'a> PartialEq for ParseError<'a> {
    fn eq(&self, rhs: &Self) -> bool {
        let left = (self.msg, self.line, self.pos, self.arg);
        let right = (rhs.msg, rhs.line, rhs.pos, rhs.arg);
        left == right
    }
}

impl<'a> fmt::Display for ParseError<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{} {}", self.line, self.pos, self.msg)?;
        if let Some(inner) = &self.source {
            write!(f, ": {}", inner)?;
        }
        if !self.arg.is_empty() {
            write!(f, ": {}", self.arg)?;
        }
        Ok(())
    }
}

impl<'a> error::Error for ParseError<'a> {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        Some(self.source.as_ref()?.as_ref())
    }
}

#[derive(Debug, PartialEq)]
pub enum SolveError {
    NotImplemented,
    PartNotFound(u8),
}

impl fmt::Display for SolveError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SolveError::NotImplemented => write!(f, "Not implemented")?,
            SolveError::PartNotFound(part) => write!(f, "Part#{part} not found")?,
        }
        Ok(())
    }
}

impl error::Error for SolveError {}

pub trait Solver {
    fn solve(&mut self, part: u8) -> Result<String, SolveError>;
}

mod y2015 {
    pub mod d1;
    pub mod d2;
    pub mod d3;
    pub mod d4;
    pub mod d5;
}

pub const PROBLEMS: [Problem; 5] = [
    y2015::d1::PROBLEM,
    y2015::d2::PROBLEM,
    y2015::d3::PROBLEM,
    y2015::d4::PROBLEM,
    y2015::d5::PROBLEM,
];

#[cfg(test)]
mod tests {
    #[test]
    fn problems_order() {
        let mut year: u16 = 0;
        let mut day: u8 = 0;
        for problem in super::PROBLEMS {
            if year < problem.year() {
                year = problem.year();
                day = 0;
            }
            assert!(year == problem.year());
            assert!(day < problem.day());
            day = problem.day();
            assert!(year >= 2015 && year < 2024);
            assert!(day >= 1 && day <= 25);
            assert_eq!(problem.parts(), 2)
        }
    }
}
