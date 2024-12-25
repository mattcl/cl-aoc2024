use std::str::FromStr;

use aoc_plumbing::Problem;

const LOCK_MASK: u32 = 0b11111;

#[derive(Debug, Clone)]
pub struct CodeChronicle {
    p1: usize,
}

impl FromStr for CodeChronicle {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut keys: Vec<u32> = Vec::with_capacity(1000);
        let mut locks: Vec<u32> = Vec::with_capacity(1000);
        for group in s.trim().split("\n\n") {
            let mut out = 0;
            for line in group.lines() {
                for ch in line.chars() {
                    out <<= 1;
                    if ch == '#' {
                        out |= 1;
                    }
                }
            }

            if out & LOCK_MASK == 0 {
                locks.push(out);
            } else {
                keys.push(out);
            }
        }

        let mut p1 = 0;

        for key in keys.iter() {
            for lock in locks.iter() {
                if key & lock == 0 {
                    p1 += 1;
                }
            }
        }

        Ok(Self { p1 })
    }
}

impl Problem for CodeChronicle {
    const DAY: usize = 25;
    const TITLE: &'static str = "code chronicle";
    const README: &'static str = include_str!("../README.md");

    type ProblemError = anyhow::Error;
    type P1 = usize;
    type P2 = i64;

    fn part_one(&mut self) -> Result<Self::P1, Self::ProblemError> {
        Ok(self.p1)
    }

    fn part_two(&mut self) -> Result<Self::P2, Self::ProblemError> {
        Ok(0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Lock {
    data: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Key {
    data: u32,
}

#[cfg(test)]
mod tests {
    use aoc_plumbing::Solution;

    use super::*;

    #[test]
    #[ignore]
    fn full_dataset() {
        let input = std::fs::read_to_string("input.txt").expect("Unable to load input");
        let solution = CodeChronicle::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(3671, 0));
    }

    #[test]
    fn example() {
        let input = "#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####";
        let solution = CodeChronicle::solve(input).unwrap();
        assert_eq!(solution, Solution::new(3, 0));
    }
}
