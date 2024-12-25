use std::str::FromStr;

use aoc_plumbing::Problem;

#[derive(Debug, Clone)]
pub struct MullItOver {
    part1: i64,
    part2: i64,
}

impl FromStr for MullItOver {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // again, easier to solve both during the parsing step because otherwise
        // I'd have to own the &str by allocating a String.

        let len = s.len();
        let bytes = s.as_bytes();
        let mut start = 0;
        let mut part1 = 0;
        let mut part2 = 0;
        let mut enable = true;

        // we get away with this because we're pretty sure the input is ascii
        'outer: while start < len {
            if bytes[start] != b'm' && bytes[start] != b'd' {
                start += 1;
                continue;
            }

            if bytes[start] == b'd' {
                if s[start..].starts_with("don't()") {
                    enable = false;
                    start += 7;
                } else if s[start..].starts_with("do()") {
                    enable = true;
                    start += 4;
                } else {
                    start += 1;
                }
                continue;
            }

            if bytes[start] == b'm' && s[start..].starts_with("mul(") {
                // left
                start += 4;
                let mut cur = start;
                let mut left = 0;
                loop {
                    if bytes[cur] == b',' {
                        break;
                    }

                    if !bytes[cur].is_ascii_digit() {
                        start = cur;
                        continue 'outer;
                    }

                    left = left * 10 + (bytes[cur] - b'0') as i64;

                    cur += 1;
                }
                if start == cur {
                    start += 1;
                    continue;
                }

                // right
                start = cur + 1;
                cur = start;
                let mut right = 0;
                loop {
                    if bytes[cur] == b')' {
                        break;
                    }

                    if !bytes[cur].is_ascii_digit() {
                        start = cur;
                        continue 'outer;
                    }

                    right = right * 10 + (bytes[cur] - b'0') as i64;
                    cur += 1;
                }
                if start == cur {
                    start += 1;
                    continue;
                }

                let prod = left * right;
                part1 += prod;
                if enable {
                    part2 += prod;
                }

                start = cur + 1;
                continue;
            }

            start += 1;
        }

        Ok(Self { part1, part2 })
    }
}

impl Problem for MullItOver {
    const DAY: usize = 3;
    const TITLE: &'static str = "mull it over";
    const README: &'static str = include_str!("../README.md");

    type ProblemError = anyhow::Error;
    type P1 = i64;
    type P2 = i64;

    fn part_one(&mut self) -> Result<Self::P1, Self::ProblemError> {
        Ok(self.part1)
    }

    fn part_two(&mut self) -> Result<Self::P2, Self::ProblemError> {
        Ok(self.part2)
    }
}

#[cfg(test)]
mod tests {
    use aoc_plumbing::Solution;

    use super::*;

    #[test]
    #[ignore]
    fn full_dataset() {
        let input = std::fs::read_to_string("input.txt").expect("Unable to load input");
        let solution = MullItOver::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(170068701, 78683433));
    }

    #[test]
    fn example() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        let solution = MullItOver::solve(input).unwrap();
        assert_eq!(solution, Solution::new(161, 48));
    }
}
