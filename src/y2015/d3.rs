use super::super::{ParseError, Problem, SolveError, Solver};

pub const PROBLEM: Problem = Problem {
    year: 2015,
    day: 3,
    title: "Perfectly Spherical Houses in a Vacuum",
    parts: 2,
    parse: |input| Ok(Box::new(Day3::new(input)?)),
};

enum Move {
    Horizontal(i8),
    Vertical(i8),
}

#[derive(Eq, Ord, PartialEq, PartialOrd)]
struct Point {
    x: isize,
    y: isize,
}

struct Day3 {
    moves: Vec<Move>,
}

impl Day3 {
    fn new<'a>(input: &'a str) -> Result<Self, ParseError<'a>> {
        let mut moves = Vec::<Move>::with_capacity(input.len());
        for (i, ch) in input.chars().enumerate() {
            match ch {
                '<' => moves.push(Move::Horizontal(-1)),
                '>' => moves.push(Move::Horizontal(1)),
                '^' => moves.push(Move::Vertical(-1)),
                'v' => moves.push(Move::Vertical(1)),
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
        }
        Ok(Day3 { moves })
    }

    fn _solve(&mut self, p: usize) -> String {
        let mut points = Vec::<Point>::with_capacity(self.moves.len() + 1);
        points.push(Point { x: 0, y: 0 });
        for (i, m) in self.moves.iter().enumerate() {
            let Point { mut x, mut y } = points[if i < p { 0 } else { i - p }];
            match *m {
                Move::Horizontal(d) => x += d as isize,
                Move::Vertical(d) => y += d as isize,
            }
            points.push(Point { x, y });
        }
        points.sort_unstable();
        let mut points = points.into_iter();
        let mut prev = points.next().unwrap();
        let mut result: usize = 1;
        for point in points {
            if point != prev {
                result += 1;
                prev = point;
            }
        }
        result.to_string()
    }
}

impl Solver for Day3 {
    fn solve(&mut self, part: u8) -> Result<String, SolveError> {
        match part {
            1 => Ok(self._solve(0)),
            2 => Ok(self._solve(1)),
            _ => Err(SolveError::PartNotFound(part)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(part: u8, input: &str, expect: &str) {
        let mut solver = Day3::new(input).unwrap();
        assert_eq!(solver.solve(part), Ok(expect.into()));
    }

    #[test]
    fn bad() {
        for input in [">\n"] {
            if let Ok(_) = Day3::new(input) {
                panic!();
            }
        }
    }

    #[test]
    fn part1() {
        test(1, ">", "2");
        test(1, "^>v<", "4");
        test(1, "^v^v^v^v^v", "2");
    }

    #[test]
    fn part2() {
        test(2, "^v", "3");
        test(2, "^>v<", "3");
        test(2, "^v^v^v^v^v", "11");
    }
}
