use std::{collections::VecDeque, str::FromStr};

use aoc_plumbing::Problem;
use aoc_std::{
    collections::{bitset::BitSet192, CharGrid},
    directions::Cardinal,
    geometry::Location,
};

// Corner checking BS
const UL: u8 = Cardinal::North as u8 | Cardinal::West as u8;
const UR: u8 = Cardinal::North as u8 | Cardinal::East as u8;
const LL: u8 = Cardinal::South as u8 | Cardinal::West as u8;
const LR: u8 = Cardinal::South as u8 | Cardinal::East as u8;

#[derive(Debug, Clone)]
pub struct GardenGroups {
    p1: u64,
    p2: u64,
}

impl FromStr for GardenGroups {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let grid = CharGrid::from_str(s)?;

        let (p1, p2) = Self::process(&grid);

        Ok(Self { p1, p2 })
    }
}

impl GardenGroups {
    pub fn process(grid: &CharGrid) -> (u64, u64) {
        let mut seen = WideGrid::new(grid.height());
        let mut queue = VecDeque::with_capacity(1000);

        let mut p1_total = 0;
        let mut p2_total = 0;

        for r in 0..grid.height() {
            for c in 0..grid.width() {
                let loc = Location::new(r, c);
                if !seen.contains(&loc) {
                    let (corners, perimeter, area) = Self::corners_area_and_perimeter(
                        &mut queue,
                        grid,
                        &loc,
                        grid.locations[r][c],
                        &mut seen,
                    );
                    p1_total += perimeter * area;
                    p2_total += corners * area;
                }
            }
        }

        (p1_total, p2_total)
    }

    // the total number of corners will be equal to the total number of sides
    fn corners_area_and_perimeter(
        cur: &mut VecDeque<Location>,
        grid: &CharGrid,
        pos: &Location,
        label: char,
        seen: &mut WideGrid,
    ) -> (u64, u64, u64) {
        cur.clear();
        cur.push_front(*pos);
        seen.insert(pos);

        let mut total_corners = 0;
        let mut perimeter = 0;
        let mut area = 0;

        while let Some(next) = cur.pop_front() {
            area += 1;

            let mut num_edges = 4;
            let mut dir_map = 0_u8;
            for (dir, neighbor_loc, neighbor_value) in grid.cardinal_neighbors(&next) {
                if neighbor_value == &label {
                    dir_map |= dir as u8;
                    num_edges -= 1;
                    if !seen.contains(&neighbor_loc) {
                        seen.insert(&neighbor_loc);
                        cur.push_back(neighbor_loc);
                    }
                }
            }

            // upper left
            let ul = UL & dir_map;
            if ul == 0 || (ul == UL && grid.locations[next.row - 1][next.col - 1] != label) {
                total_corners += 1;
            }

            // upper right
            let ur = UR & dir_map;
            if ur == 0 || (ur == UR && grid.locations[next.row - 1][next.col + 1] != label) {
                total_corners += 1;
            }

            // lower left
            let ll = LL & dir_map;
            if ll == 0 || (ll == LL && grid.locations[next.row + 1][next.col - 1] != label) {
                total_corners += 1;
            }

            // lower right
            let lr = LR & dir_map;
            if lr == 0 || (lr == LR && grid.locations[next.row + 1][next.col + 1] != label) {
                total_corners += 1;
            }

            perimeter += num_edges;
        }

        (total_corners, perimeter, area)
    }
}

impl Problem for GardenGroups {
    const DAY: usize = 12;
    const TITLE: &'static str = "garden groups";
    const README: &'static str = include_str!("../README.md");

    type ProblemError = anyhow::Error;
    type P1 = u64;
    type P2 = u64;

    fn part_one(&mut self) -> Result<Self::P1, Self::ProblemError> {
        Ok(self.p1)
    }

    fn part_two(&mut self) -> Result<Self::P2, Self::ProblemError> {
        Ok(self.p2)
    }
}

#[derive(Debug, Clone)]
pub struct WideGrid {
    rows: Vec<BitSet192>,
}

impl WideGrid {
    pub fn new(height: usize) -> Self {
        Self {
            rows: vec![BitSet192::ZERO; height],
        }
    }

    pub fn insert(&mut self, location: &Location) {
        self.rows[location.row].insert(location.col);
    }

    pub fn contains(&self, location: &Location) -> bool {
        self.rows[location.row].contains(location.col)
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
        let solution = GardenGroups::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(1465968, 897702));
    }

    #[test]
    fn example() {
        let input = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";
        let solution = GardenGroups::solve(input).unwrap();
        assert_eq!(solution, Solution::new(1930, 1206));
    }

    #[test]
    fn example2() {
        let input = "XXX";
        let solution = GardenGroups::solve(input).unwrap();
        assert_eq!(solution, Solution::new(24, 12));
    }

    #[test]
    fn example3() {
        let input = "XXX
XXX
XXX";
        let solution = GardenGroups::solve(input).unwrap();
        assert_eq!(solution, Solution::new(108, 36));
    }

    #[test]
    fn example4() {
        let input = "XAX
AAA
XAX";
        let solution = GardenGroups::solve(input).unwrap();
        assert_eq!(solution, Solution::new(76, 76));
    }

    #[test]
    fn example5() {
        let input = "
XAX
AXA
XAX";
        let solution = GardenGroups::solve(input).unwrap();
        assert_eq!(solution, Solution::new(36, 36));
    }
}
