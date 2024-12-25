use std::str::FromStr;

use anyhow::anyhow;
use aoc_plumbing::Problem;
use nom::{character::complete, combinator, multi::separated_list1, IResult};

#[derive(Debug, Clone)]
pub struct PrintQueue {
    rules_left: [u128; 100],
    rules_right: [u128; 100],
    updates: Vec<Update>,
}

impl FromStr for PrintQueue {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (rules, update_str) = s
            .trim()
            .split_once("\n\n")
            .ok_or_else(|| anyhow!("invalid input"))?;

        let mut rules_left = [0; 100];
        let mut rules_right = [0; 100];

        for raw_rule in rules.lines() {
            let (left_raw, right_raw) = raw_rule
                .split_once('|')
                .ok_or_else(|| anyhow!("invalid input"))?;
            let left: u8 = left_raw.parse()?;
            let right: u8 = right_raw.parse()?;

            rules_left[left as usize] |= 1 << right;
            rules_right[right as usize] |= 1 << left;
        }

        let (_, updates) = parse_updates(update_str).map_err(|e| e.to_owned())?;

        Ok(Self {
            rules_left,
            rules_right,
            updates,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Update {
    pages: Vec<u8>,
}

impl Update {
    pub fn is_valid(&self, rules: &[u128]) -> bool {
        let mut seen_before = 0_u128;
        for page in self.pages.iter() {
            let left = rules[*page as usize];

            // we have a rule for PAGE|X
            if left != 0 && seen_before & left != 0 {
                // we're not good
                return false;
            }

            let mask = 1_u128 << page;
            seen_before |= mask;
        }

        true
    }

    pub fn middle(&self) -> u8 {
        self.pages[self.pages.len() / 2]
    }

    // we don't need to actually re-order the list, we just need to know what
    // the middle number _would_ be
    pub fn middle_reorder(&self, rules_left: &[u128], rules_right: &[u128]) -> u8 {
        let each_side = (self.pages.len() / 2) as u32;

        let mut seen = 0_u128;
        for page in self.pages.iter().copied() {
            seen |= 1_u128 << page;
        }

        for page in self.pages.iter().copied() {
            let cur = seen & !(1_u128 << page);

            let rule = rules_left[page as usize];

            if rule == 0 || (cur & rule).count_ones() != each_side {
                continue;
            }

            let rule = rules_right[page as usize];

            if rule == 0 || (cur & rule).count_ones() != each_side {
                continue;
            }

            return page;
        }

        // for this problem to have unambiguous solutions, this must be true
        // that we can never reach this line.
        unreachable!()
    }
}

fn parse_updates(input: &str) -> IResult<&str, Vec<Update>> {
    separated_list1(complete::newline, parse_update)(input)
}

fn parse_update(input: &str) -> IResult<&str, Update> {
    combinator::map(
        separated_list1(complete::char(','), complete::u8),
        |pages| Update { pages },
    )(input)
}

impl Problem for PrintQueue {
    const DAY: usize = 5;
    const TITLE: &'static str = "print queue";
    const README: &'static str = include_str!("../README.md");

    type ProblemError = anyhow::Error;
    type P1 = u64;
    type P2 = u64;

    fn part_one(&mut self) -> Result<Self::P1, Self::ProblemError> {
        Ok(self
            .updates
            .iter()
            .filter(|&u| u.is_valid(&self.rules_left))
            .map(|u| u.middle() as u64)
            .sum())
    }

    fn part_two(&mut self) -> Result<Self::P2, Self::ProblemError> {
        Ok(self
            .updates
            .iter()
            .filter(|&u| !u.is_valid(&self.rules_left))
            .map(|u| u.middle_reorder(&self.rules_left, &self.rules_right) as u64)
            .sum())
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
        let solution = PrintQueue::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(4996, 6311));
    }

    #[test]
    fn example() {
        let input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
        let solution = PrintQueue::solve(input).unwrap();
        assert_eq!(solution, Solution::new(143, 123));
    }
}
