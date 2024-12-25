use std::{collections::hash_map::Entry, str::FromStr};

use aoc_plumbing::Problem;
use aoc_std::geometry::Point2D;
use itertools::Itertools;
use num::integer::gcd;
use rustc_hash::FxHashMap;

#[derive(Debug, Clone)]
pub struct ResonantCollinearity {
    antennas: FxHashMap<u8, Vec<Point2D<i8>>>,
    size: i8,
}

impl FromStr for ResonantCollinearity {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut antennas: FxHashMap<u8, Vec<Point2D<i8>>> = FxHashMap::default();

        let mut size = 0;
        for (r, line) in s.trim().lines().enumerate() {
            size = line.len() as i8;
            for (c, ch) in line.chars().enumerate() {
                if ch != '.' {
                    match antennas.entry(ch as u8) {
                        Entry::Occupied(mut occupied_entry) => {
                            occupied_entry.get_mut().push((c as i8, r as i8).into());
                        }
                        Entry::Vacant(vacant_entry) => {
                            let s = vacant_entry.insert(Vec::default());
                            s.push((c as i8, r as i8).into());
                        }
                    }
                }
            }
        }

        Ok(Self { antennas, size })
    }
}

impl ResonantCollinearity {
    pub fn compute_antinodes(&self) -> usize {
        let mut antinodes = AntinodeGrid::default();

        for antennas in self.antennas.values() {
            self.compute_antinodes_for(antennas, &mut antinodes);
        }

        antinodes.count()
    }

    fn compute_antinodes_for(&self, antennas: &[Point2D<i8>], antinodes: &mut AntinodeGrid) {
        for (a, b) in antennas.iter().tuple_combinations() {
            let left = a.min(b);
            let right = a.max(b);
            let slope = right - left;

            let candidate1 = left - slope;
            let candidate2 = right + slope;

            if candidate1.x >= 0
                && candidate1.x < self.size
                && candidate1.y >= 0
                && candidate1.y < self.size
            {
                antinodes.insert(&candidate1);
            }

            if candidate2.x >= 0
                && candidate2.x < self.size
                && candidate2.y >= 0
                && candidate2.y < self.size
            {
                antinodes.insert(&candidate2);
            }
        }
    }

    pub fn compute_line_antinodes(&self) -> usize {
        let mut antinodes = AntinodeGrid::default();

        for antennas in self.antennas.values() {
            self.compute_line_antinodes_for(antennas, &mut antinodes);
        }

        antinodes.count()
    }

    fn compute_line_antinodes_for(&self, antennas: &[Point2D<i8>], antinodes: &mut AntinodeGrid) {
        for (a, b) in antennas.iter().tuple_combinations() {
            let left = a.min(b);
            let right = a.max(b);
            let mut slope = right - left;

            // So this never gets triggered in a real input, but it must be a
            // general possibility
            loop {
                let d = gcd(slope.x, slope.y);

                if d == 1 {
                    break;
                }

                slope.x /= d;
                slope.y /= d;
            }

            // unlike for part 1, we branch off from a single node, and walk to
            // the edges of the grid.
            let mut candidate1 = left - slope;
            let mut candidate2 = left + slope;

            antinodes.insert(left);

            loop {
                if candidate1.x >= 0
                    && candidate1.x < self.size
                    && candidate1.y >= 0
                    && candidate1.y < self.size
                {
                    antinodes.insert(&candidate1);
                } else {
                    break;
                }

                candidate1 -= slope;
            }

            loop {
                if candidate2.x >= 0
                    && candidate2.x < self.size
                    && candidate2.y >= 0
                    && candidate2.y < self.size
                {
                    antinodes.insert(&candidate2);
                } else {
                    break;
                }
                candidate2 += slope;
            }
        }
    }
}

impl Problem for ResonantCollinearity {
    const DAY: usize = 8;
    const TITLE: &'static str = "resonant collinearity";
    const README: &'static str = include_str!("../README.md");

    type ProblemError = anyhow::Error;
    type P1 = usize;
    type P2 = usize;

    fn part_one(&mut self) -> Result<Self::P1, Self::ProblemError> {
        Ok(self.compute_antinodes())
    }

    fn part_two(&mut self) -> Result<Self::P2, Self::ProblemError> {
        Ok(self.compute_line_antinodes())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AntinodeGrid {
    grid: [u64; 50],
}

impl Default for AntinodeGrid {
    fn default() -> Self {
        Self { grid: [0; 50] }
    }
}

impl AntinodeGrid {
    pub fn insert(&mut self, point: &Point2D<i8>) {
        self.grid[point.y as usize] |= 1 << point.x as usize;
    }

    pub fn count(&self) -> usize {
        self.grid.iter().map(|r| r.count_ones()).sum::<u32>() as usize
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
        let solution = ResonantCollinearity::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(413, 1417));
    }

    #[test]
    fn example() {
        let input = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";
        let solution = ResonantCollinearity::solve(input).unwrap();
        assert_eq!(solution, Solution::new(14, 34));
    }
}
