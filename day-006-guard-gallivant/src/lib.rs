use std::str::FromStr;

use aoc_plumbing::Problem;
use aoc_std::{
    collections::{CharGrid, Grid},
    directions::Cardinal,
    geometry::Location,
};
use rayon::prelude::*;
use rustc_hash::{FxBuildHasher, FxHashMap, FxHashSet};

#[derive(Debug, Clone)]
pub struct GuardGallivant {
    guard: Guard,
    grid: Grid<char>,
    obstacles_rows: Vec<WideMap>,
    obstacles_cols: Vec<WideMap>,
    candidate_states: FxHashMap<Guard, Location>,
}

impl FromStr for GuardGallivant {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut grid = CharGrid::from_str(s)?;
        let mut guard = Guard {
            location: Location::default(),
            facing: Cardinal::North,
        };
        let mut obstacles_rows = vec![WideMap::default(); grid.height()];
        let mut obstacles_cols = vec![WideMap::default(); grid.width()];

        #[allow(clippy::needless_range_loop)]
        for r in 0..grid.height() {
            for c in 0..grid.width() {
                let ch = grid.locations[r][c];
                if ch == '^' {
                    guard.location = Location::new(r, c);
                    grid.locations[r][c] = '.';
                } else if ch == '#' {
                    obstacles_rows[r].insert(c);
                    obstacles_cols[c].insert(r);
                }
            }
        }

        Ok(Self {
            guard,
            grid,
            obstacles_rows,
            obstacles_cols,
            candidate_states: FxHashMap::default(),
        })
    }
}

impl GuardGallivant {
    /// check if the specified configuration produces a loop
    fn valid_configuration(&self, mut guard: Guard, obstruction: Location) -> bool {
        let mut seen = FxHashSet::with_capacity_and_hasher(100, FxBuildHasher);
        seen.insert(guard);
        loop {
            // given our current position and heading, get the next obstacle
            // ahead of us
            match guard.facing {
                Cardinal::North => {
                    let mut row_map = self.obstacles_rows[guard.location.row];
                    if obstruction.row == guard.location.row {
                        row_map.insert(obstruction.col);
                    }

                    if let Some(next_col) = row_map.next_right(guard.location.col) {
                        guard.location.col = next_col;
                    } else {
                        return false;
                    }
                }
                Cardinal::South => {
                    let mut row_map = self.obstacles_rows[guard.location.row];
                    if obstruction.row == guard.location.row {
                        row_map.insert(obstruction.col);
                    }

                    if let Some(next_col) = row_map.next_left(guard.location.col) {
                        guard.location.col = next_col;
                    } else {
                        return false;
                    }
                }
                Cardinal::East => {
                    let mut col_map = self.obstacles_cols[guard.location.col];
                    if obstruction.col == guard.location.col {
                        col_map.insert(obstruction.row);
                    }

                    if let Some(next_row) = col_map.next_right(guard.location.row) {
                        guard.location.row = next_row;
                    } else {
                        return false;
                    }
                }
                Cardinal::West => {
                    let mut col_map = self.obstacles_cols[guard.location.col];
                    if obstruction.col == guard.location.col {
                        col_map.insert(obstruction.row);
                    }

                    if let Some(next_row) = col_map.next_left(guard.location.row) {
                        guard.location.row = next_row;
                    } else {
                        return false;
                    }
                }
            }

            guard.facing = guard.facing.right();

            if !seen.insert(guard) {
                return true;
            }
        }
    }
}

impl Problem for GuardGallivant {
    const DAY: usize = 6;
    const TITLE: &'static str = "guard gallivant";
    const README: &'static str = include_str!("../README.md");

    type ProblemError = anyhow::Error;
    type P1 = usize;
    type P2 = usize;

    fn part_one(&mut self) -> Result<Self::P1, Self::ProblemError> {
        let mut seen_locations = FxHashSet::default();
        seen_locations.insert(self.guard.location);

        let mut guard = self.guard;
        while let Some((loc, ch)) = self.grid.cardinal_neighbor(&guard.location, guard.facing) {
            match ch {
                '#' => guard.facing = guard.facing.right(),
                _ => {
                    // if we haven't already seen the location we're going to
                    // we can add our _current_ configuration to the list of
                    // candidate states
                    if !seen_locations.contains(&loc) {
                        // we need this for part 2
                        self.candidate_states.insert(guard, loc);
                        seen_locations.insert(loc);
                    }
                    guard.location = loc;
                }
            }
        }

        Ok(seen_locations.len())
    }

    fn part_two(&mut self) -> Result<Self::P2, Self::ProblemError> {
        // okay, we now know the set of all candidate states, so let's check
        // each one of them
        let count = self
            .candidate_states
            .par_iter()
            .filter(|(state, obstruction)| self.valid_configuration(**state, **obstruction))
            .count();

        Ok(count)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Guard {
    location: Location,
    facing: Cardinal,
}

/// Sigh, the whole 130x130 grid is a PITA. It might be better to make this
/// align better, but I can't be bothered to do the math with more than two
/// values.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WideMap {
    left: u128,
    right: u8,
}

impl WideMap {
    pub fn insert(&mut self, idx: usize) {
        if idx < 128 {
            self.left |= 1 << idx;
        } else {
            self.right |= 1 << (idx - 128);
        }
    }

    /// get the next open space prior to an obstacle to our right, if one exists
    pub fn next_right(&self, idx: usize) -> Option<usize> {
        if idx < 128 {
            if idx < 127 {
                let shifted = self.left >> (idx + 1);
                if shifted > 0 {
                    let offset = shifted.trailing_zeros() as usize;
                    return Some(idx + offset);
                }
            }
            if self.right > 0 {
                let right_offset = self.right.trailing_zeros() as usize;
                return Some(127 + right_offset);
            }
        } else {
            let shifted = self.right >> (idx - 128 + 1);
            if shifted > 0 {
                let offset = shifted.trailing_zeros() as usize;
                return Some(idx + offset);
            }
        }

        None
    }

    /// get the next open space prior to an obstacle to our left, if one exists
    pub fn next_left(&self, idx: usize) -> Option<usize> {
        if idx < 128 {
            if idx == 0 {
                return None;
            }
            let shifted = self.left << (128 - idx);
            if shifted > 0 {
                let offset = shifted.leading_zeros() as usize;
                return Some(idx - offset);
            }
        } else {
            if idx > 128 {
                let shifted = self.right << (8 - (idx - 128));
                if shifted > 0 {
                    let offset = shifted.leading_zeros() as usize;
                    return Some(idx - offset);
                }
            }
            if self.left > 0 {
                let left_offset = self.left.leading_zeros() as usize;
                return Some(128 - left_offset);
            }
        }

        None
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
        let solution = GuardGallivant::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(4663, 1530));
    }

    #[test]
    fn example() {
        let input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
        let solution = GuardGallivant::solve(input).unwrap();
        assert_eq!(solution, Solution::new(41, 6));
    }
}
