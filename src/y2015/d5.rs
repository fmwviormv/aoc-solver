use super::super::{ParseError, Problem, SolveError, Solver};

pub const PROBLEM: Problem = Problem {
    year: 2015,
    day: 5,
    title: "Doesn't He Have Intern-Elves For This?",
    parts: 2,
    parse: |input| Ok(Box::new(Day5::new(input)?)),
};

struct Day5 {
    input: Vec<String>,
}

impl Day5 {
    fn new<'a>(input: &'a str) -> Result<Self, ParseError<'a>> {
        Ok(Day5 {
            input: input.lines().map(|line| line.to_string()).collect(),
        })
    }

    fn part1(&mut self) -> String {
        let mut result: usize = 0;
        for text in &self.input {
            if aei(text) >= 3 && xx(text) > 0 && !bad(text) {
                result += 1;
            }
        }
        result.to_string()
    }

    fn part2(&mut self) -> String {
        let mut result: usize = 0;
        for text in &self.input {
            if xyxy(text) > 0 && xyx(text) > 0 {
                result += 1;
            }
        }
        result.to_string()
    }
}

impl Solver for Day5 {
    fn solve(&mut self, part: u8) -> Result<String, SolveError> {
        match part {
            1 => Ok(self.part1()),
            2 => Ok(self.part2()),
            _ => Err(SolveError::PartNotFound(part)),
        }
    }
}

fn aei(text: &str) -> usize {
    let mut result: usize = 0;
    for char in text.chars() {
        result += match char {
            'a' | 'e' | 'i' | 'o' | 'u' => 1,
            _ => 0,
        }
    }
    result
}

fn xx(text: &str) -> usize {
    let mut result: usize = 0;
    let mut chars = text.chars();
    let mut prev = {
        match chars.next() {
            None => return result,
            Some(char) => char,
        }
    };
    for char in chars {
        result += if char == prev { 1 } else { 0 };
        prev = char;
    }
    result
}

fn bad(text: &str) -> bool {
    for x in ["ab", "cd", "pq", "xy"] {
        if text.contains(x) {
            return true;
        }
    }
    false
}

fn xyxy(text: &str) -> usize {
    let mut text = text.chars();
    let mut prev = match text.next() {
        None => return 0,
        Some(x) => x,
    };
    let mut list: Vec<(String, usize)> = text
        .enumerate()
        .map(|(i, next)| {
            let text = format!("{prev}{next}");
            prev = next;
            (text, i)
        })
        .collect();
    list.sort();
    let mut result: usize = 0;
    let mut prev = ("".to_string(), 0);
    for next in list {
        if prev.0 == next.0 && prev.1 + 1 < next.1 {
            result += 1;
        }
        prev = next;
    }
    result
}

fn xyx(text: &str) -> usize {
    let mut text = text.chars();
    let mut a = match text.next() {
        None => return 0,
        Some(x) => x,
    };
    let mut b = match text.next() {
        None => return 0,
        Some(x) => x,
    };
    let mut result: usize = 0;
    for c in text {
        if a == c {
            result += 1;
        }
        (a, b) = (b, c);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(part: u8, input: &str, expect: &str) {
        let mut solver = Day5::new(input).unwrap();
        assert_eq!(solver.solve(part), Ok(expect.into()));
    }

    #[test]
    fn utils() {
        assert_eq!(aei("aei"), 3);
        assert_eq!(aei("xazegov"), 3);
        assert_eq!(aei("aeiouaeiouaeiou"), 15);
        assert_eq!(xx("xx"), 1);
        assert_eq!(xx("abcdde"), 1);
        assert_eq!(xx("aabbccdd"), 4);
        assert_eq!(xyxy("xyxy"), 1);
        assert_eq!(xyxy("aabcdefgaa"), 1);
        assert_eq!(xyxy("aaa"), 0);
        assert_eq!(xyx("xyx"), 1);
        assert_eq!(xyx("abcdefeghi"), 1);
        assert_eq!(xyx("aaa"), 1);
    }

    #[test]
    fn part1() {
        let text = "ugknbfddgicrmopn";
        assert_eq!(aei(text), 3);
        assert_eq!(xx(text), 1);
        assert!(!bad(text));
        test(1, text, "1");
        let text = "aaa";
        assert_eq!(aei(text), 3);
        assert_eq!(xx(text), 2);
        assert!(!bad(text));
        test(1, text, "1");
        let text = "jchzalrnumimnmhp";
        assert_eq!(xx(text), 0);
        test(1, text, "0");
        let text = "haegwjzuvuyypxyu";
        assert!(bad(text));
        test(1, text, "0");
        let text = "dvszwmarrgswjxmb";
        assert_eq!(aei(text), 1);
        test(1, text, "0");
    }

    #[test]
    fn part2() {
        let text = "qjhvhtzxzqqjkmpb";
        assert_eq!(xyxy(text), 1);
        assert_eq!(xyx(text), 2);
        test(2, text, "1");
        let text = "xxyxx";
        assert_eq!(xyxy(text), 1);
        assert_eq!(xyx(text), 1);
        test(2, text, "1");
        let text = "uurcxstgmygtbstg";
        assert_eq!(xyxy(text), 2);
        assert_eq!(xyx(text), 0);
        test(2, text, "0");
        let text = "ieodomkazucvgmuy";
        assert_eq!(xyxy(text), 0);
        assert_eq!(xyx(text), 1);
        test(2, text, "0");
    }
}
