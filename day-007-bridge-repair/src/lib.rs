use std::str::FromStr;

use aoc_plumbing::Problem;
use nom::{
    bytes::complete::tag,
    character::complete::{self, multispace1, space1},
    combinator,
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};
use rayon::prelude::*;

#[derive(Debug, Clone)]
pub struct BridgeRepair {
    p1: i64,
    p2: i64,
}

impl FromStr for BridgeRepair {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, equations) = parse_equations(s).map_err(|e| e.to_owned())?;

        let (p1, p2) = equations
            .into_par_iter()
            .map(|eq| eq.is_valid_combined_unwind_dfs())
            .reduce(|| (0, 0), |(p1, p2), (a1, a2)| (p1 + a1, p2 + a2));

        Ok(Self { p1, p2 })
    }
}

impl Problem for BridgeRepair {
    const DAY: usize = 7;
    const TITLE: &'static str = "bridge repair";
    const README: &'static str = include_str!("../README.md");

    type ProblemError = anyhow::Error;
    type P1 = i64;
    type P2 = i64;

    fn part_one(&mut self) -> Result<Self::P1, Self::ProblemError> {
        Ok(self.p1)
    }

    fn part_two(&mut self) -> Result<Self::P2, Self::ProblemError> {
        Ok(self.p2)
    }
}

#[derive(Debug, Clone)]
pub struct Equation {
    left: i64,
    right: Vec<i64>,
    widths: Vec<i64>,
}

impl Equation {
    pub fn is_valid_combined_unwind_dfs(&self) -> (i64, i64) {
        let mut p1_valid = false;
        let mut p2_valid = false;
        self._is_valid_combined_unwind_dfs(
            self.right.len(),
            self.left,
            false,
            &mut p1_valid,
            &mut p2_valid,
        );

        if p1_valid {
            (self.left, self.left)
        } else if p2_valid {
            (0, self.left)
        } else {
            (0, 0)
        }
    }

    fn _is_valid_combined_unwind_dfs(
        &self,
        remaining: usize,
        head: i64,
        used_concat: bool,
        p1_valid: &mut bool,
        p2_valid: &mut bool,
    ) {
        if remaining == 0 {
            if head == 0 {
                *p2_valid = true;
                if !used_concat {
                    *p1_valid = true;
                }
            }
            return;
        }

        if head < 0 || (*p2_valid && used_concat) {
            return;
        }

        let idx = remaining - 1;
        let v = self.right[idx];

        if head % v == 0 {
            self._is_valid_combined_unwind_dfs(idx, head / v, used_concat, p1_valid, p2_valid);
            if *p1_valid {
                return;
            }
        }

        self._is_valid_combined_unwind_dfs(idx, head - v, used_concat, p1_valid, p2_valid);

        if *p1_valid {
            return;
        }

        if !*p2_valid {
            let width = self.widths[idx];
            if head % width == v {
                self._is_valid_combined_unwind_dfs(idx, head / width, true, p1_valid, p2_valid);
            }
        }
    }
}

fn digits(val: i64) -> u8 {
    (val.checked_ilog10().unwrap_or(0) + 1) as u8
}

fn parse_equations(input: &str) -> IResult<&str, Vec<Equation>> {
    separated_list1(multispace1, parse_equation)(input)
}

fn parse_equation(input: &str) -> IResult<&str, Equation> {
    combinator::map(
        separated_pair(
            complete::i64,
            tag(": "),
            separated_list1(space1, complete::i64),
        ),
        |(left, right)| {
            let widths = right
                .iter()
                .map(|v| 10_i64.pow(digits(*v) as u32))
                .collect();
            Equation {
                left,
                right,
                widths,
            }
        },
    )(input)
}

#[cfg(test)]
mod tests {
    use aoc_plumbing::Solution;

    use super::*;

    #[test]
    #[ignore]
    fn full_dataset() {
        let input = std::fs::read_to_string("input.txt").expect("Unable to load input");
        let solution = BridgeRepair::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(2299996598890, 362646859298554));
    }

    #[test]
    fn example() {
        let input = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";
        let solution = BridgeRepair::solve(input).unwrap();
        assert_eq!(solution, Solution::new(3749, 11387));
    }
}
