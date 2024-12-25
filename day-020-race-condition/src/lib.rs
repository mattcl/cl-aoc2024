use std::str::FromStr;

use aoc_plumbing::Problem;
use aoc_std::{
    collections::CharGrid,
    geometry::{AocPoint, Location, Point2D},
};
use rayon::prelude::*;

#[derive(Debug, Clone)]
pub struct RaceConditionGen<const N: i32> {
    p1: usize,
    p2: usize,
}

impl<const N: i32> FromStr for RaceConditionGen<N> {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut grid = CharGrid::from_str(s)?;

        let mut cur = Location::new(0, 0);

        #[allow(clippy::needless_range_loop)]
        'outer: for r in 1..(grid.height() - 1) {
            for c in 1..(grid.width() - 1) {
                if grid.locations[r][c] == 'S' {
                    cur = Location::new(r, c);
                    break 'outer;
                }
            }
        }

        let mut path = Vec::with_capacity(5000);

        loop {
            let v = grid.get(&cur).unwrap();

            path.push(Point2D::new(cur.row as i32, cur.col as i32));

            if *v == 'E' {
                break;
            }

            grid.locations[cur.row][cur.col] = '#';

            for (_dir, nloc, nv) in grid.cardinal_neighbors(&cur) {
                if *nv == '#' {
                    continue;
                }

                cur = nloc;
                break;
            }
        }

        let Point2D { x: p1, y: p2 } = (0..(path.len() - 1))
            .into_par_iter()
            .map(|i| {
                let mut p1 = 0;
                let mut p2 = 0;

                let iloc = path[i];

                let mut j = i + N as usize + 1;
                while j < path.len() {
                    let jloc = path[j];
                    let dist = iloc.manhattan_dist(&jloc);
                    if dist < 21 && (j - i) as i32 - dist > N {
                        if dist == 2 {
                            p1 += 1;
                        }
                        p2 += 1;
                    } else if dist > 20 {
                        // we know that we can jump ahead by at least this much
                        j += dist as usize - 20;
                        continue;
                    }
                    j += 1;
                }

                (p1, p2).into()
            })
            .sum();

        Ok(Self { p1, p2 })
    }
}

impl<const N: i32> Problem for RaceConditionGen<N> {
    const DAY: usize = 20;
    const TITLE: &'static str = "race condition";
    const README: &'static str = include_str!("../README.md");

    type ProblemError = anyhow::Error;
    type P1 = usize;
    type P2 = usize;

    fn part_one(&mut self) -> Result<Self::P1, Self::ProblemError> {
        Ok(self.p1)
    }

    fn part_two(&mut self) -> Result<Self::P2, Self::ProblemError> {
        Ok(self.p2)
    }
}

pub type RaceCondition = RaceConditionGen<99>;

#[cfg(test)]
mod tests {
    use aoc_plumbing::Solution;

    use super::*;

    #[test]
    #[ignore]
    fn full_dataset() {
        let input = std::fs::read_to_string("input.txt").expect("Unable to load input");
        let solution = RaceCondition::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(1452, 999556));
    }

    #[test]
    fn example() {
        let input = "###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";
        let solution = RaceConditionGen::<49>::solve(input).unwrap();
        assert_eq!(solution, Solution::new(1, 285));
    }
}
