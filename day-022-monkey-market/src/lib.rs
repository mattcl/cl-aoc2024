use std::str::FromStr;

use aoc_plumbing::Problem;
use nom::{
    character::complete::{self, newline},
    multi::separated_list1,
    IResult,
};
use rayon::prelude::*;

// -9 Ob00000  0
// -8 Ob00001  1
// -7 Ob00010  2
// -6 Ob00011  3
// -5 Ob00100  4
// -4 Ob00101  5
// -3 Ob00110  6
// -2 Ob00111  7
// -1 Ob01000  8
//  0 Ob01001  9
//  1 Ob01010  10
//  2 Ob01011  11
//  3 Ob01100  12
//  4 Ob01101  13
//  5 Ob01110  14
//  6 Ob01111  15
//  7 Ob10000  16
//  8 Ob10001  17
//  9 Ob10010  18
//
//  so, really, we only need a few numbers beyond
//  so the biggest number that only needs 19 bits would be
//                          6    3    0    0
// const SM_SEQ_MAX: usize = 0b01111011000100101001;

// N % 16,777,216 is equal to N & MOD_MASK;
const MOD_MASK: u64 = (1 << 24) - 1;
const SEQ_MASK: usize = (1 << 20) - 1;
// Under our encoding scheme, the max value is the following sequence
//                       9     0     0     0
const SEQ_MAX: usize = 0b10010_01001_01001_01001;
// and the minimum is   -9     0     0     0
const SEQ_MIN: usize = 0b00000_01001_01001_01001;
const SEQ_SIZE: usize = SEQ_MAX + 1 - SEQ_MIN;
const DESIRED_CHUNKS: usize = 4;

#[derive(Debug, Clone)]
pub struct MonkeyMarket {
    p1: u64,
    p2: u16,
}

impl FromStr for MonkeyMarket {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, initial_numbers) = parse_numbers(s).map_err(|e| e.to_owned())?;

        let chunk_size = (initial_numbers.len() / DESIRED_CHUNKS)
            + if initial_numbers.len() % DESIRED_CHUNKS == 0 {
                0
            } else {
                1
            };
        let (p1, p2, _totals) = initial_numbers
            .par_chunks(chunk_size)
            .map(|chunk| {
                let mut totals = vec![0_u16; SEQ_SIZE];
                let mut seen = vec![usize::MAX; SEQ_SIZE];
                let mut num_total = 0;

                for (i, n) in chunk.iter().enumerate() {
                    let mut cur = *n;
                    let mut key: usize = 0;
                    let mut prev = (cur % 10) as i8;

                    for j in 0..2000 {
                        cur = next_number(cur);
                        let cur_digit = (cur % 10) as i8;
                        let delta: i8 = cur_digit - prev;
                        prev = cur_digit;
                        key = ((key << 5) & SEQ_MASK) | (delta + 9) as usize;

                        let adjusted_key = key - SEQ_MIN;

                        if j > 2 && seen[adjusted_key] != i {
                            seen[adjusted_key] = i;
                            totals[adjusted_key] += cur_digit as u16;
                        }
                    }
                    num_total += cur;
                }

                (num_total, 0, totals)
            })
            .reduce(
                || (0_u64, 0_u16, vec![0_u16; SEQ_SIZE]),
                |(mut total_num, mut best, mut acc),
                 (chunk_total_num, chunk_best, chunk_totals)| {
                    total_num += chunk_total_num;
                    best = best.max(chunk_best);
                    for i in 0..acc.len() {
                        acc[i] += chunk_totals[i];
                        best = best.max(acc[i]);
                    }
                    (total_num, best, acc)
                },
            );

        Ok(Self { p1, p2 })
    }
}

#[inline]
fn next_number(input: u64) -> u64 {
    let mut a = (input ^ (input << 6)) & MOD_MASK;
    a = a ^ (a >> 5);
    a ^ ((a << 11) & MOD_MASK)
}

fn parse_numbers(input: &str) -> IResult<&str, Vec<u64>> {
    separated_list1(newline, complete::u64)(input)
}

impl Problem for MonkeyMarket {
    const DAY: usize = 22;
    const TITLE: &'static str = "monkey market";
    const README: &'static str = include_str!("../README.md");

    type ProblemError = anyhow::Error;
    type P1 = u64;
    type P2 = u16;

    fn part_one(&mut self) -> Result<Self::P1, Self::ProblemError> {
        Ok(self.p1)
    }

    fn part_two(&mut self) -> Result<Self::P2, Self::ProblemError> {
        Ok(self.p2)
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
        let solution = MonkeyMarket::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(17965282217, 2152));
    }

    // #[test]
    // fn example1() {
    //     let input = "1
    // 10
    // 100
    // 2024";
    //     let solution = MonkeyMarket::solve(input).unwrap();
    //     assert_eq!(solution, Solution::new(37327623, 24));
    // }

    // #[test]
    // fn example2() {
    //     let input = "1
    // 2
    // 3
    // 2024";
    //     let solution = MonkeyMarket::solve(input).unwrap();
    //     assert_eq!(solution, Solution::new(37990510, 23));
    // }

    #[test]
    fn verify_next() {
        assert_eq!(100000000 & MOD_MASK, 16113920);
        assert_eq!(next_number(123), 15887950);

        let mut a = 1;

        for _ in 0..2000 {
            a = next_number(a);
            // println!("{:024b}", a);
        }

        // assert!(false);

        assert_eq!(a, 8685429);
    }
}
