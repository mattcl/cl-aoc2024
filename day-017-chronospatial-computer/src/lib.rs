use std::{fmt::Display, str::FromStr};

use anyhow::anyhow;
use aoc_plumbing::Problem;
use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{self},
    multi::separated_list1,
    sequence::preceded,
    IResult,
};

#[derive(Debug, Clone)]
pub struct ChronospatialComputer {
    a: u64,
    v1: u64,
    v3: u64,
    program: Vec<u64>,
}

impl FromStr for ChronospatialComputer {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (raw_registers, raw_program) = s
            .split_once("\n\n")
            .ok_or_else(|| anyhow!("invalid input"))?;

        // we only need to know A
        let (_, a) = parse_registers(raw_registers).map_err(|e| e.to_owned())?;
        let (_, program) = parse_program(raw_program).map_err(|e| e.to_owned())?;

        //   for my input,
        //   0 bst 4  b = a & 0b111
        //   2 bxl 3  b = b ^ 3
        //   4 cdv 5  c = a >> b
        //   6 bxc 1  b = b ^ c
        //   8 bxl 3  b = b ^ 3
        //  10 adv 3  a = a >> 3
        //  12 out 5  out b & 0b111
        //  14 jnz 0  goto 0 if a != 0
        //
        // i've been told that the position of the second bxl varies a bit
        //
        // this is the example expected locations from my input
        // expected = [2, 4, 1, v1, 7, 5, 4, v2, 1, v3, 0, 3, 5, 5, 3, 0];

        let v1 = program[3];
        let v3 = {
            let mut ip = 4;
            while ip < program.len() - 1 {
                if program[ip] == 1 {
                    break;
                }
                ip += 2;
            }
            program[ip + 1]
        };

        Ok(Self { a, v1, v3, program })
    }
}

impl Problem for ChronospatialComputer {
    const DAY: usize = 17;
    const TITLE: &'static str = "chronospatial computer";
    const README: &'static str = include_str!("../README.md");

    type ProblemError = anyhow::Error;
    type P1 = String;
    type P2 = u64;

    fn part_one(&mut self) -> Result<Self::P1, Self::ProblemError> {
        let mut out: Vec<u64> = Vec::with_capacity(1000);

        let mut a = self.a;

        while a != 0 {
            out.push(transpiled_digit(a, self.v1, self.v3));
            a >>= 3;
        }

        Ok(out.into_iter().join(","))
    }

    fn part_two(&mut self) -> Result<Self::P2, Self::ProblemError> {
        let mut cur = Vec::with_capacity(1000);
        let mut next = Vec::with_capacity(1000);

        cur.push(0);

        for wanted in self.program.iter().rev() {
            for p in cur.drain(..) {
                for i in 0_u64..8 {
                    let a = (p << 3) + i;
                    if transpiled_digit(a, self.v1, self.v3) == *wanted {
                        next.push(a);
                    }
                }
            }
            std::mem::swap(&mut cur, &mut next);
        }

        cur.sort();

        for c in cur {
            let mut digit = 0;
            let mut a = c;

            while a > 0 && digit < self.program.len() {
                digit += 1;
                a >>= 3;
            }

            if digit == self.program.len() {
                return Ok(c);
            }
        }

        Ok(0)
    }
}

/// this is what the loop of the program does
fn transpiled_digit(a: u64, v1: u64, v3: u64) -> u64 {
    let b = (a & 0b111) ^ v1;
    ((b ^ (a >> b)) ^ v3) & 0b111
}

fn parse_program(input: &str) -> IResult<&str, Vec<u64>> {
    preceded(
        tag("Program: "),
        separated_list1(complete::char(','), complete::u64),
    )(input)
}

fn parse_registers(input: &str) -> IResult<&str, u64> {
    preceded(tag("Register A: "), complete::u64)(input)
}

/// Was used for parsing the original problem so it would be easier to reverse-
/// engineer
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Opcode {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

impl Display for Opcode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Opcode::Adv => "adv",
            Opcode::Bxl => "bxl",
            Opcode::Bst => "bst",
            Opcode::Jnz => "jnz",
            Opcode::Bxc => "bxc",
            Opcode::Out => "out",
            Opcode::Bdv => "bdv",
            Opcode::Cdv => "cdv",
        }
        .fmt(f)
    }
}

impl From<u64> for Opcode {
    fn from(value: u64) -> Self {
        match value {
            0 => Self::Adv,
            1 => Self::Bxl,
            2 => Self::Bst,
            3 => Self::Jnz,
            4 => Self::Bxc,
            5 => Self::Out,
            6 => Self::Bdv,
            7 => Self::Cdv,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Operand {
    Literal(u64),
    A,
    B,
    C,
}

impl From<u64> for Operand {
    fn from(value: u64) -> Self {
        match value {
            0..=3 => Self::Literal(value),
            4 => Self::A,
            5 => Self::B,
            6 => Self::C,
            _ => unreachable!(),
        }
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
        let solution = ChronospatialComputer::solve(&input).unwrap();
        assert_eq!(
            solution,
            Solution::new("1,5,3,0,2,5,2,5,3".into(), 108107566389757)
        );
    }

    // #[test]
    // fn example() {
    //     let input = "Register A: 9641146161661
    // Register B: 0
    // Register C: 0

    // Program: 2,4,1,3,7,5,4,1,1,3,0,3,5,5,3,0";
    //     let solution = ChronospatialComputer::solve(input).unwrap();
    //     assert_eq!(solution, Solution::new("2,4,1,3,7,5,4,1,1,3,0,3,5,5,3,0".into(), 0));
    // }
}
