use std::str::FromStr;

use anyhow::anyhow;
use aoc_plumbing::Problem;

#[derive(Debug, Clone)]
pub struct RedNosedReports {
    part_1_count: usize,
    part_2_count: usize,
}

impl FromStr for RedNosedReports {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut part_1_count = 0;
        let mut part_2_count = 0;
        let mut buffer = Vec::with_capacity(20);
        for line in s.trim().lines() {
            let (p1, p2) = process_line(line.trim(), &mut buffer)?;
            buffer.clear();

            if p1 {
                part_1_count += 1;
            }

            if p2 {
                part_2_count += 1;
            }
        }
        // usually i'd prefer _not_ solving the problem during the parsing, but
        // in this case it was almost necessary to avoid the allocations
        Ok(Self {
            part_1_count,
            part_2_count,
        })
    }
}

fn process_line(input: &str, buffer: &mut Vec<Candidate>) -> anyhow::Result<(bool, bool)> {
    let mut parts = input.split(' ');

    let first: i8 = parts
        .next()
        .ok_or_else(|| anyhow!("empty report"))?
        .parse()?;
    if let Some(second_raw) = parts.next() {
        let mut first_candidate = Candidate::new(first);
        let second: i8 = second_raw.parse()?;

        // this is as if we'd skipped the first value
        buffer.push(Candidate::new(second));
        // this was skipping the second value
        buffer.push(Candidate::new(first));

        first_candidate.push(second);

        for part in parts {
            let val: i8 = part.parse()?;

            buffer.retain_mut(|c| c.push(val));

            if first_candidate.valid {
                buffer.push(first_candidate);
                first_candidate.push(val);
            } else if buffer.is_empty() {
                return Ok((false, false));
            }
        }

        if first_candidate.valid {
            Ok((true, true))
        } else {
            Ok((false, buffer.iter().any(|c| c.valid)))
        }
    } else {
        Ok((true, true))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Candidate {
    last: i8,
    delta_dir: i8,
    valid: bool,
}

impl Candidate {
    pub fn new(value: i8) -> Self {
        Self {
            last: value,
            delta_dir: 0,
            valid: true,
        }
    }

    pub fn push(&mut self, value: i8) -> bool {
        let delta = self.last - value;
        self.last = value;

        if delta == 0 {
            self.valid = false;
            return false;
        }

        if delta.abs() > 3 {
            self.valid = false;
            return false;
        }

        if self.delta_dir == 0 {
            self.delta_dir = delta.signum();
            true
        } else if delta.signum() != self.delta_dir {
            self.valid = false;
            false
        } else {
            true
        }
    }
}

impl Problem for RedNosedReports {
    const DAY: usize = 2;
    const TITLE: &'static str = "red nosed reports";
    const README: &'static str = include_str!("../README.md");

    type ProblemError = anyhow::Error;
    type P1 = usize;
    type P2 = usize;

    fn part_one(&mut self) -> Result<Self::P1, Self::ProblemError> {
        Ok(self.part_1_count)
    }

    fn part_two(&mut self) -> Result<Self::P2, Self::ProblemError> {
        Ok(self.part_2_count)
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
        let solution = RedNosedReports::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(332, 398));
    }

    #[test]
    fn example() {
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        let solution = RedNosedReports::solve(input).unwrap();
        assert_eq!(solution, Solution::new(2, 4));
    }
}
