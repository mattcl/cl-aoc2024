use std::{collections::BTreeSet, str::FromStr};

use anyhow::anyhow;
use aoc_plumbing::Problem;
use aoc_std::{
    collections::CharGrid,
    directions::{BoundedCardinalNeighbors, Cardinal, Direction},
    geometry::Location,
};

#[derive(Debug, Clone)]
pub struct WarehouseWoes {
    grid: CharGrid,
    wide_grid: CharGrid,
    start: Location,
    movements: Vec<Cardinal>,
}

impl FromStr for WarehouseWoes {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (left, right) = s
            .trim()
            .split_once("\n\n")
            .ok_or_else(|| anyhow!("invalid input"))?;

        let mut grid = CharGrid::from_str(left)?;

        let mut start = Location::default();

        'outer: for r in 0..grid.height() {
            for c in 0..grid.width() {
                if grid.locations[r][c] == '@' {
                    start = Location::new(r, c);
                    grid.locations[r][c] = '.';
                    break 'outer;
                }
            }
        }

        let mut wide_grid = CharGrid::from(vec![vec!['.'; grid.width() * 2]; grid.height()]);

        for r in 0..grid.height() {
            for c in 0..grid.width() {
                let wc = c * 2;
                match grid.locations[r][c] {
                    '#' => {
                        wide_grid.locations[r][wc + 1] = '#';
                        wide_grid.locations[r][wc] = '#';
                    }
                    'O' => {
                        wide_grid.locations[r][wc + 1] = ']';
                        wide_grid.locations[r][wc] = '[';
                    }
                    _ => {}
                }
            }
        }

        let movements = right
            .lines()
            .flat_map(|line| line.chars())
            .map(|c| match c {
                '<' => Cardinal::West,
                '>' => Cardinal::East,
                '^' => Cardinal::North,
                'v' => Cardinal::South,
                _ => panic!("what is this {}", c),
            })
            .collect();

        Ok(Self {
            grid,
            wide_grid,
            start,
            movements,
        })
    }
}

impl WarehouseWoes {
    pub fn rearrange(&mut self) -> usize {
        let mut pos = self.start;

        for m in 0..self.movements.len() {
            let dir = self.movements[m];
            if let Some(next) = self.maybe_shift_boxes(&pos, dir) {
                pos = next;
            }
        }

        let mut out = 0;

        for r in 1..self.grid.height() - 1 {
            for c in 1..self.grid.width() - 1 {
                if self.grid.locations[r][c] == 'O' {
                    out += r * 100 + c;
                }
            }
        }

        out
    }

    pub fn rearrange_wide(&mut self) -> usize {
        let mut pos = self.start;
        pos.col *= 2;

        for m in 0..self.movements.len() {
            let dir = self.movements[m];
            if let Some(next) = self.maybe_shift_wide_boxes(&pos, dir) {
                pos = next;
            }
        }

        let mut out = 0;

        for r in 1..self.wide_grid.height() - 1 {
            for c in 2..self.wide_grid.width() - 2 {
                if self.wide_grid.locations[r][c] == '[' {
                    out += r * 100 + c;
                }
            }
        }

        out
    }

    fn maybe_shift_boxes(&mut self, loc: &Location, direction: Cardinal) -> Option<Location> {
        if let Some((nloc, nch)) = self.grid.cardinal_neighbor(loc, direction) {
            match nch {
                '.' => return Some(nloc),
                'O' => {
                    if let Some(last) = self.maybe_shift_dir(&nloc, direction) {
                        self.grid.set(&last, 'O').unwrap();
                        self.grid.set(&nloc, '.').unwrap();
                        return Some(nloc);
                    }
                }
                _ => return None,
            }
        }
        None
    }

    fn maybe_shift_dir(&self, loc: &Location, direction: Cardinal) -> Option<Location> {
        if let Some((nloc, nch)) = self.grid.cardinal_neighbor(loc, direction) {
            return match nch {
                '.' => Some(nloc),
                'O' => self.maybe_shift_dir(&nloc, direction),
                _ => None,
            };
        }
        None
    }

    fn maybe_shift_wide_boxes(&mut self, loc: &Location, direction: Cardinal) -> Option<Location> {
        if let Some((nloc, nch)) = self.wide_grid.cardinal_neighbor(loc, direction) {
            match nch {
                '.' => return Some(nloc),
                '[' => {
                    match direction {
                        // easy
                        Cardinal::East => {
                            let mut seen = Vec::default();
                            seen.push(nloc);
                            if self.maybe_shift_east(&nloc, &mut seen) {
                                for s in seen.iter().rev() {
                                    self.wide_grid.locations[s.row][s.col + 2] = ']';
                                    self.wide_grid.locations[s.row][s.col + 1] = '[';
                                }
                                self.wide_grid.locations[nloc.row][nloc.col] = '.';
                                return Some(nloc);
                            }
                        }

                        Cardinal::North => {
                            let right = nloc.cardinal_neighbor(Cardinal::East).unwrap();
                            let mut seen = BTreeSet::new();
                            if self.maybe_shift_north(&nloc, &right, &mut seen) {
                                for s in seen {
                                    self.wide_grid.locations[s.row][s.col] = '.';
                                    self.wide_grid.locations[s.row - 1][s.col] = '[';

                                    self.wide_grid.locations[s.row][s.col + 1] = '.';
                                    self.wide_grid.locations[s.row - 1][s.col + 1] = ']';
                                }

                                self.wide_grid.locations[nloc.row][nloc.col] = '.';
                                self.wide_grid.locations[nloc.row - 1][nloc.col] = '[';

                                self.wide_grid.locations[right.row][right.col] = '.';
                                self.wide_grid.locations[right.row - 1][right.col] = ']';
                                return Some(nloc);
                            }
                        }
                        Cardinal::South => {
                            let right = nloc.cardinal_neighbor(Cardinal::East).unwrap();
                            let mut seen = BTreeSet::new();
                            if self.maybe_shift_south(&nloc, &right, &mut seen) {
                                for s in seen.iter().rev() {
                                    self.wide_grid.locations[s.row][s.col] = '.';
                                    self.wide_grid.locations[s.row + 1][s.col] = '[';

                                    self.wide_grid.locations[s.row][s.col + 1] = '.';
                                    self.wide_grid.locations[s.row + 1][s.col + 1] = ']';
                                }

                                self.wide_grid.locations[nloc.row][nloc.col] = '.';
                                self.wide_grid.locations[nloc.row + 1][nloc.col] = '[';

                                self.wide_grid.locations[right.row][right.col] = '.';
                                self.wide_grid.locations[right.row + 1][right.col] = ']';
                                return Some(nloc);
                            }
                        }
                        // should ot be possible
                        Cardinal::West => unreachable!(),
                    }
                }
                ']' => {
                    match direction {
                        // easy
                        Cardinal::West => {
                            let mut seen = Vec::default();
                            seen.push(nloc);
                            if self.maybe_shift_west(&nloc, &mut seen) {
                                for s in seen.iter().rev() {
                                    self.wide_grid.locations[s.row][s.col - 2] = '[';
                                    self.wide_grid.locations[s.row][s.col - 1] = ']';
                                }
                                self.wide_grid.locations[nloc.row][nloc.col] = '.';
                                return Some(nloc);
                            }
                        }

                        Cardinal::North => {
                            let left = nloc.cardinal_neighbor(Cardinal::West).unwrap();
                            let mut seen = BTreeSet::new();
                            if self.maybe_shift_north(&left, &nloc, &mut seen) {
                                for s in seen {
                                    self.wide_grid.locations[s.row][s.col] = '.';
                                    self.wide_grid.locations[s.row - 1][s.col] = '[';

                                    self.wide_grid.locations[s.row][s.col + 1] = '.';
                                    self.wide_grid.locations[s.row - 1][s.col + 1] = ']';
                                }

                                self.wide_grid.locations[left.row][left.col] = '.';
                                self.wide_grid.locations[left.row - 1][left.col] = '[';

                                self.wide_grid.locations[nloc.row][nloc.col] = '.';
                                self.wide_grid.locations[nloc.row - 1][nloc.col] = ']';
                                return Some(nloc);
                            }
                        }
                        Cardinal::South => {
                            let left = nloc.cardinal_neighbor(Cardinal::West).unwrap();
                            let mut seen = BTreeSet::new();
                            if self.maybe_shift_south(&left, &nloc, &mut seen) {
                                for s in seen.iter().rev() {
                                    self.wide_grid.locations[s.row][s.col] = '.';
                                    self.wide_grid.locations[s.row + 1][s.col] = '[';

                                    self.wide_grid.locations[s.row][s.col + 1] = '.';
                                    self.wide_grid.locations[s.row + 1][s.col + 1] = ']';
                                }
                                self.wide_grid.locations[left.row][left.col] = '.';
                                self.wide_grid.locations[left.row + 1][left.col] = '[';

                                self.wide_grid.locations[nloc.row][nloc.col] = '.';
                                self.wide_grid.locations[nloc.row + 1][nloc.col] = ']';
                                return Some(nloc);
                            }
                        }
                        // should ot be possible
                        Cardinal::East => unreachable!(),
                    }
                }
                _ => return None,
            }
        }
        None
    }

    fn maybe_shift_east(&self, loc: &Location, seen: &mut Vec<Location>) -> bool {
        // we know these are safe because the grid is already padded
        let nloc = loc.project(&Direction::East, 2).unwrap();
        let nch = self.wide_grid.get(&nloc).unwrap();

        match nch {
            '.' => true,
            '[' => {
                seen.push(nloc);
                self.maybe_shift_east(&nloc, seen)
            }
            _ => false,
        }
    }

    fn maybe_shift_west(&self, loc: &Location, seen: &mut Vec<Location>) -> bool {
        // we know these are safe because the grid is already padded
        let nloc = loc.project(&Direction::West, 2).unwrap();
        let nch = self.wide_grid.get(&nloc).unwrap();

        match nch {
            '.' => true,
            ']' => {
                seen.push(nloc);
                self.maybe_shift_west(&nloc, seen)
            }
            _ => false,
        }
    }

    fn maybe_shift_north(
        &self,
        left: &Location,
        right: &Location,
        seen: &mut BTreeSet<Location>,
    ) -> bool {
        if let (Some((l_loc, l_ch)), Some((r_loc, r_ch))) = (
            self.wide_grid.cardinal_neighbor(left, Cardinal::North),
            self.wide_grid.cardinal_neighbor(right, Cardinal::North),
        ) {
            return match (l_ch, r_ch) {
                ('.', '.') => true,
                ('[', ']') => !seen.insert(l_loc) || self.maybe_shift_north(&l_loc, &r_loc, seen),
                (']', '.') => {
                    let left = l_loc.cardinal_neighbor(Cardinal::West).unwrap();
                    !seen.insert(left) || self.maybe_shift_north(&left, &l_loc, seen)
                }
                ('.', '[') => {
                    !seen.insert(r_loc)
                        || self.maybe_shift_north(
                            &r_loc,
                            &r_loc.cardinal_neighbor(Cardinal::East).unwrap(),
                            seen,
                        )
                }
                (']', '[') => {
                    let left = l_loc.cardinal_neighbor(Cardinal::West).unwrap();

                    if seen.insert(left) && !self.maybe_shift_north(&left, &l_loc, seen) {
                        return false;
                    }

                    if seen.insert(r_loc) {
                        self.maybe_shift_north(
                            &r_loc,
                            &r_loc.cardinal_neighbor(Cardinal::East).unwrap(),
                            seen,
                        )
                    } else {
                        true
                    }
                }
                _ => false,
            };
        }
        false
    }

    fn maybe_shift_south(
        &self,
        left: &Location,
        right: &Location,
        seen: &mut BTreeSet<Location>,
    ) -> bool {
        if let (Some((l_loc, l_ch)), Some((r_loc, r_ch))) = (
            self.wide_grid.cardinal_neighbor(left, Cardinal::South),
            self.wide_grid.cardinal_neighbor(right, Cardinal::South),
        ) {
            return match (l_ch, r_ch) {
                ('.', '.') => true,
                ('[', ']') => !seen.insert(l_loc) || self.maybe_shift_south(&l_loc, &r_loc, seen),
                (']', '.') => {
                    let left = l_loc.cardinal_neighbor(Cardinal::West).unwrap();
                    !seen.insert(left) || self.maybe_shift_south(&left, &l_loc, seen)
                }
                ('.', '[') => {
                    !seen.insert(r_loc)
                        || self.maybe_shift_south(
                            &r_loc,
                            &r_loc.cardinal_neighbor(Cardinal::East).unwrap(),
                            seen,
                        )
                }
                (']', '[') => {
                    let left = l_loc.cardinal_neighbor(Cardinal::West).unwrap();

                    if seen.insert(left) && !self.maybe_shift_south(&left, &l_loc, seen) {
                        return false;
                    }

                    if seen.insert(r_loc) {
                        self.maybe_shift_south(
                            &r_loc,
                            &r_loc.cardinal_neighbor(Cardinal::East).unwrap(),
                            seen,
                        )
                    } else {
                        true
                    }
                }
                _ => false,
            };
        }
        false
    }
}

impl Problem for WarehouseWoes {
    const DAY: usize = 15;
    const TITLE: &'static str = "warehouse woes";
    const README: &'static str = include_str!("../README.md");

    type ProblemError = anyhow::Error;
    type P1 = usize;
    type P2 = usize;

    fn part_one(&mut self) -> Result<Self::P1, Self::ProblemError> {
        Ok(self.rearrange())
    }

    fn part_two(&mut self) -> Result<Self::P2, Self::ProblemError> {
        Ok(self.rearrange_wide())
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
        let solution = WarehouseWoes::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(1421727, 1463160));
    }

    #[test]
    fn example() {
        let input = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";
        let solution = WarehouseWoes::solve(input).unwrap();
        assert_eq!(solution, Solution::new(10092, 9021));
    }

    // #[test]
    // fn example2() {
    //     let input = "#######
    // #...#.#
    // #.....#
    // #..OO@#
    // #..O..#
    // #.....#
    // #######

    // <vv<<^^<<^^";
    //     let solution = WarehouseWoes::solve(input).unwrap();
    //     assert_eq!(solution, Solution::new(10092, 9021));
    // }
}
