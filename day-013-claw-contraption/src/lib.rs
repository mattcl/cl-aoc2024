use std::str::FromStr;

use aoc_plumbing::Problem;
use aoc_std::geometry::Point2D;
use nom::{
    bytes::complete::tag,
    character::complete::{self, multispace0, newline},
    combinator,
    multi::many1,
    sequence::{self, preceded, separated_pair, terminated},
    IResult,
};

#[derive(Debug, Clone)]
pub struct ClawContraption {
    machines: Vec<Machine>,
}

impl FromStr for ClawContraption {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, machines) = parse_machines(s).map_err(|e| e.to_owned())?;
        Ok(Self { machines })
    }
}

impl Problem for ClawContraption {
    const DAY: usize = 13;
    const TITLE: &'static str = "claw contraption";
    const README: &'static str = include_str!("../README.md");

    type ProblemError = anyhow::Error;
    type P1 = i64;
    type P2 = i64;

    fn part_one(&mut self) -> Result<Self::P1, Self::ProblemError> {
        Ok(self.machines.iter().filter_map(|m| m.cost_small()).sum())
    }

    fn part_two(&mut self) -> Result<Self::P2, Self::ProblemError> {
        Ok(self.machines.iter().filter_map(|m| m.cost_large()).sum())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Machine {
    a: Point2D<i64>,
    b: Point2D<i64>,
    prize: Point2D<i64>,
}

impl Machine {
    // a.x * n + b.x * m = prize.x
    // a.y * m + b.y * m = prize.y
    pub fn cost_small(&self) -> Option<i64> {
        let det = self.a.x * self.b.y - self.a.y * self.b.x;
        if det == 0 {
            return None;
        }

        let n = self.prize.x * self.b.y - self.prize.y * self.b.x;
        let m = self.a.x * self.prize.y - self.a.y * self.prize.x;

        if n % det != 0 || m % det != 0 {
            return None;
        }

        Some(n / det * 3 + m / det)
    }

    pub fn cost_large(&self) -> Option<i64> {
        let prize = self.prize + 10_000_000_000_000;
        let det = self.a.x * self.b.y - self.a.y * self.b.x;
        if det == 0 {
            return None;
        }

        let n = prize.x * self.b.y - prize.y * self.b.x;
        let m = self.a.x * prize.y - self.a.y * prize.x;

        if n % det != 0 || m % det != 0 {
            return None;
        }

        Some(n / det * 3 + m / det)
    }
}

fn parse_machines(input: &str) -> IResult<&str, Vec<Machine>> {
    many1(parse_machine)(input)
}

fn parse_machine(input: &str) -> IResult<&str, Machine> {
    combinator::map(
        sequence::tuple((
            terminated(parse_button_a, newline),
            terminated(parse_button_b, newline),
            terminated(parse_prize, multispace0),
        )),
        |(a, b, prize)| Machine { a, b, prize },
    )(input)
}

fn parse_button_a(input: &str) -> IResult<&str, Point2D<i64>> {
    preceded(tag("Button A: X+"), parse_coord)(input)
}

fn parse_button_b(input: &str) -> IResult<&str, Point2D<i64>> {
    preceded(tag("Button B: X+"), parse_coord)(input)
}

fn parse_coord(input: &str) -> IResult<&str, Point2D<i64>> {
    combinator::map(
        separated_pair(complete::i64, tag(", Y+"), complete::i64),
        |(x, y)| Point2D::new(x, y),
    )(input)
}

fn parse_prize(input: &str) -> IResult<&str, Point2D<i64>> {
    combinator::map(
        preceded(
            tag("Prize: X="),
            separated_pair(complete::i64, tag(", Y="), complete::i64),
        ),
        |(x, y)| Point2D::new(x, y),
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
        let solution = ClawContraption::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(35997, 82510994362072));
    }

    #[test]
    fn example() {
        let input = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";
        let solution = ClawContraption::solve(input).unwrap();
        assert_eq!(solution, Solution::new(480, 875318608908));
    }
}
