use std::str::FromStr;

use aoc_plumbing::Problem;
use aoc_std::{collections::Grid, directions::Direction, geometry::Location};

#[derive(Debug, Clone)]
pub struct CeresSearch {
    grid: Grid<char>,
    x_pos: Vec<Location>,
    a_pos: Vec<Location>,
}

impl FromStr for CeresSearch {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut raw = vec![vec!['.'; 140]; 140];
        let mut x_pos = Vec::with_capacity(5000);
        let mut a_pos = Vec::with_capacity(5000);

        for (r, line) in s.trim().lines().enumerate() {
            for (c, v) in line.chars().enumerate() {
                if v == 'X' {
                    x_pos.push(Location::new(r, c));
                } else if v == 'A' {
                    a_pos.push(Location::new(r, c));
                }
                raw[r][c] = v;
            }
        }

        let grid = Grid::new(raw);

        Ok(Self { grid, x_pos, a_pos })
    }
}

impl CeresSearch {
    fn count_xmas(&self) -> usize {
        self.x_pos.iter().map(|loc| self.count_xmas_from(loc)).sum()
    }

    fn count_xmas_from(&self, location: &Location) -> usize {
        self.grid
            .neighbors(location)
            .filter(|(dir, loc, v)| **v == 'M' && self.search_xmas_dir(loc, dir))
            .count()
    }

    fn search_xmas_dir(&self, location: &Location, dir: &Direction) -> bool {
        location
            .project(dir, 2)
            .and_then(|l| self.grid.get(&l))
            .filter(|v| **v == 'S')
            .and_then(|_| location.project(dir, 1))
            .and_then(|l| self.grid.get(&l))
            .map(|v| *v == 'A')
            .unwrap_or_default()
    }

    fn count_mas_x(&self) -> usize {
        self.a_pos.iter().filter(|loc| self.is_x_from(loc)).count()
    }

    fn is_x_from(&self, location: &Location) -> bool {
        match (
            location
                .project(&Direction::NorthEast, 1)
                .and_then(|l| self.grid.get(&l)),
            location
                .project(&Direction::SouthWest, 1)
                .and_then(|l| self.grid.get(&l)),
        ) {
            (Some('M'), Some('S')) | (Some('S'), Some('M')) => {
                matches!(
                    (
                        location
                            .project(&Direction::NorthWest, 1)
                            .and_then(|l| self.grid.get(&l)),
                        location
                            .project(&Direction::SouthEast, 1)
                            .and_then(|l| self.grid.get(&l))
                    ),
                    (Some('M'), Some('S')) | (Some('S'), Some('M'))
                )
            }
            _ => false,
        }
    }
}

impl Problem for CeresSearch {
    const DAY: usize = 4;
    const TITLE: &'static str = "ceres search";
    const README: &'static str = include_str!("../README.md");

    type ProblemError = anyhow::Error;
    type P1 = usize;
    type P2 = usize;

    fn part_one(&mut self) -> Result<Self::P1, Self::ProblemError> {
        Ok(self.count_xmas())
    }

    fn part_two(&mut self) -> Result<Self::P2, Self::ProblemError> {
        Ok(self.count_mas_x())
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
        let solution = CeresSearch::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(2573, 1850));
    }

    #[test]
    fn example() {
        let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        let solution = CeresSearch::solve(input).unwrap();
        assert_eq!(solution, Solution::new(18, 9));
    }
}
