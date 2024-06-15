use super::super::{ParseError, Problem, SolveError, Solver};

pub const PROBLEM: Problem = Problem {
    year: 2015,
    day: 4,
    title: "The Ideal Stocking Stuffer",
    parts: 2,
    parse: |input| Ok(Box::new(Day4::new(input)?)),
};

struct Day4 {
    input: String,
}

impl Day4 {
    fn new<'a>(input: &'a str) -> Result<Self, ParseError<'a>> {
        if input.len() > 20 {
            return Err(ParseError {
                msg: "Too long",
                line: 1,
                pos: 1,
                source: None,
                arg: input,
            });
        }
        Ok(Day4 {
            input: input.to_string(),
        })
    }

    fn mask(&mut self, m: u32) -> String {
        let mut i: usize = 0;
        loop {
            i = i.wrapping_add(1);
            if i == 0 {
                panic!("Overflow!");
            }
            let text = format!("{}{}", self.input, i);
            let a = md5a(&text.into_bytes());
            if (a & m) == 0 {
                return i.to_string();
            }
        }
    }
}

impl Solver for Day4 {
    fn solve(&mut self, part: u8) -> Result<String, SolveError> {
        match part {
            1 => Ok(self.mask(0xf0ffff)),
            2 => Ok(self.mask(0xffffff)),
            _ => Err(SolveError::PartNotFound(part)),
        }
    }
}

fn md5a(input: &[u8]) -> u32 {
    const S: [u8; 64] = [
        7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, 5, 9, 14, 20, 5, 9, 14, 20, 5,
        9, 14, 20, 5, 9, 14, 20, 4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23, 6, 10,
        15, 21, 6, 10, 15, 21, 6, 10, 15, 21, 6, 10, 15, 21,
    ];
    const K: [u32; 64] = [
        0xd76aa478, 0xe8c7b756, 0x242070db, 0xc1bdceee, 0xf57c0faf, 0x4787c62a, 0xa8304613,
        0xfd469501, 0x698098d8, 0x8b44f7af, 0xffff5bb1, 0x895cd7be, 0x6b901122, 0xfd987193,
        0xa679438e, 0x49b40821, 0xf61e2562, 0xc040b340, 0x265e5a51, 0xe9b6c7aa, 0xd62f105d,
        0x02441453, 0xd8a1e681, 0xe7d3fbc8, 0x21e1cde6, 0xc33707d6, 0xf4d50d87, 0x455a14ed,
        0xa9e3e905, 0xfcefa3f8, 0x676f02d9, 0x8d2a4c8a, 0xfffa3942, 0x8771f681, 0x6d9d6122,
        0xfde5380c, 0xa4beea44, 0x4bdecfa9, 0xf6bb4b60, 0xbebfbc70, 0x289b7ec6, 0xeaa127fa,
        0xd4ef3085, 0x04881d05, 0xd9d4d039, 0xe6db99e5, 0x1fa27cf8, 0xc4ac5665, 0xf4292244,
        0x432aff97, 0xab9423a7, 0xfc93a039, 0x655b59c3, 0x8f0ccc92, 0xffeff47d, 0x85845dd1,
        0x6fa87e4f, 0xfe2ce6e0, 0xa3014314, 0x4e0811a1, 0xf7537e82, 0xbd3af235, 0x2ad7d2bb,
        0xeb86d391,
    ];
    const I: (u32, u32, u32, u32) = (0x67452301, 0xefcdab89, 0x98badcfe, 0x10325476);
    let len = input.len();
    if len >= 56 {
        panic!("Too big");
    }
    let len: u32 = len as u32;
    let mut m: Vec<u8> = input.into();
    m.push(128);
    m.resize(56, 0u8);
    let m = {
        let mut result = [0u32; 16];
        for (i, x) in m.into_iter().enumerate() {
            result[i >> 2] |= (x as u32) << (8 * (i & 3));
        }
        result[14] = 8 * len;
        result
    };
    let (mut a, mut b, mut c, mut d) = I;
    for i in 0..64usize {
        let (f, g): (u32, usize) = if i < 32 {
            if i < 16 {
                ((b & c) | (!b & d), i)
            } else {
                ((d & b) | (!d & c), (5 * i + 1) & 15)
            }
        } else {
            if i < 48 {
                (b ^ c ^ d, (3 * i + 5) & 15)
            } else {
                (c ^ (b | !d), (7 * i) & 15)
            }
        };
        let x = f.wrapping_add(a).wrapping_add(K[i]).wrapping_add(m[g]);
        a = d;
        d = c;
        c = b;
        let y = S[i];
        b = b.wrapping_add((x << y) | (x >> (32 - y)));
    }
    a.wrapping_add(I.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(part: u8, input: &str, expect: &str) {
        let mut solver = Day4::new(input).unwrap();
        assert_eq!(solver.solve(part), Ok(expect.into()));
    }

    #[test]
    fn bad() {
        for input in ["123456789012345678901234567890"] {
            if let Ok(_) = Day4::new(input) {
                panic!();
            }
        }
    }

    #[test]
    fn utils() {
        assert_eq!(md5a("abcdef609043".as_bytes()), 0xdb010000);
        assert_eq!(md5a("pqrstuv1048970".as_bytes()), 0x13060000);
    }

    #[test]
    #[ignore]
    fn part1() {
        test(1, "abcdef", "609043");
        test(1, "pqrstuv", "1048970");
    }
}
