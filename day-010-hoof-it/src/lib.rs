use std::str::FromStr;

use aoc_plumbing::Problem;
use aoc_std::{
    collections::{DigitGrid, Grid},
    geometry::{Location, Point2D},
};
use rustc_hash::FxHashSet;

#[derive(Debug, Clone)]
pub struct HoofIt {
    p1: u16,
    p2: u16,
}

impl FromStr for HoofIt {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let grid = DigitGrid::from_str(s)?;

        let Point2D { x: p1, y: p2 } = Self::sum_trailheads(&grid);

        Ok(Self { p1, p2 })
    }
}

impl HoofIt {
    pub fn sum_trailheads(grid: &DigitGrid) -> Point2D<u16> {
        let mut cache = Grid::from(vec![vec![None; grid.width()]; grid.height()]);

        let mut sum = Point2D::default();
        for r in 0..grid.height() {
            for c in 0..grid.width() {
                if grid.locations[r][c] == 0 {
                    let origin = Location::new(r, c);
                    let total = Self::_sum_trailheads(grid, origin, &mut cache);
                    sum.x += cache
                        .get(&origin)
                        .map(|v| v.as_ref().map(|e| e.unique.len()).unwrap_or_default())
                        .unwrap_or_default() as u16;
                    sum.y += total;
                }
            }
        }
        sum
    }

    fn _sum_trailheads(
        grid: &DigitGrid,
        pos: Location,
        cache: &mut Grid<Option<CacheEntry>>,
    ) -> u16 {
        if let Some(cur) = grid.get(&pos).copied() {
            if cur == 9 {
                let mut out = FxHashSet::default();
                out.insert(pos);
                cache
                    .set(
                        &pos,
                        Some(CacheEntry {
                            unique: out,
                            total: 1,
                        }),
                    )
                    .unwrap();
                return 1;
            }

            if let Some(Some(cached)) = cache.get(&pos) {
                return cached.total;
            }

            let mut out = FxHashSet::default();
            let mut total = 0;
            for (_dir, neighbor_loc, neighbor_val) in grid.cardinal_neighbors(&pos) {
                if cur + 1 == *neighbor_val {
                    total += Self::_sum_trailheads(grid, neighbor_loc, cache);
                    // cloning in this instance would be way cheaper than having
                    // to compute a bunch of hashes on extend.
                    if out.is_empty() {
                        out = cache.locations[neighbor_loc.row][neighbor_loc.col]
                            .as_ref()
                            .unwrap()
                            .unique
                            .clone();
                    } else {
                        out.extend(
                            cache.locations[neighbor_loc.row][neighbor_loc.col]
                                .as_ref()
                                .unwrap()
                                .unique
                                .iter()
                                .copied(),
                        );
                    }
                }
            }

            cache
                .set(&pos, Some(CacheEntry { unique: out, total }))
                .unwrap();
            total
        } else {
            0
        }
    }
}

#[derive(Debug, Clone)]
struct CacheEntry {
    unique: FxHashSet<Location>,
    total: u16,
}

impl Problem for HoofIt {
    const DAY: usize = 10;
    const TITLE: &'static str = "hoof it";
    const README: &'static str = include_str!("../README.md");

    type ProblemError = anyhow::Error;
    type P1 = u16;
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
        let solution = HoofIt::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(624, 1483));
    }

    #[test]
    fn example() {
        let input = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";
        let solution = HoofIt::solve(input).unwrap();
        assert_eq!(solution, Solution::new(36, 81));
    }
}
