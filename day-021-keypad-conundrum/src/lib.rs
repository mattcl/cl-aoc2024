use std::{rc::Rc, str::FromStr};

use aoc_plumbing::Problem;
use aoc_std::geometry::Point2D;
use itertools::Itertools;
use rustc_hash::FxHashMap;
use xxhash_rust::xxh3::xxh3_64;

// we can fit 16 moves in a single u64
// 0b0000 0
// 0b0001 1
// 0b0010 2
// 0b0011 3
// 0b0100 4
// 0b0101 5
// 0b0110 6
// 0b0111 7
// 0b1000 8
// 0b1001 9
// 0b1010 A
// 0b1011 ^
// 0b1100 v
// 0b1101 <
// 0b1110 >

const DIGIT_HOLE: Point2D<i8> = Point2D { x: 0, y: 3 };

const fn digit_pos(digit: u8) -> Point2D<i8> {
    match digit {
        b'7' => Point2D { x: 0, y: 0 },
        b'8' => Point2D { x: 1, y: 0 },
        b'9' => Point2D { x: 2, y: 0 },
        b'4' => Point2D { x: 0, y: 1 },
        b'5' => Point2D { x: 1, y: 1 },
        b'6' => Point2D { x: 2, y: 1 },
        b'1' => Point2D { x: 0, y: 2 },
        b'2' => Point2D { x: 1, y: 2 },
        b'3' => Point2D { x: 2, y: 2 },
        b'0' => Point2D { x: 1, y: 3 },
        b'A' => Point2D { x: 2, y: 3 },
        _ => unreachable!(),
    }
}

const NAV_HOLE: Point2D<i8> = Point2D { x: 0, y: 0 };

const fn nav_pos(ch: u8) -> Point2D<i8> {
    match ch {
        b'^' => Point2D { x: 1, y: 0 },
        b'A' => Point2D { x: 2, y: 0 },
        b'<' => Point2D { x: 0, y: 1 },
        b'v' => Point2D { x: 1, y: 1 },
        b'>' => Point2D { x: 2, y: 1 },
        _ => unreachable!(),
    }
}

const fn direction(ch: u8) -> Point2D<i8> {
    match ch {
        b'^' => Point2D { x: 0, y: -1 },
        b'<' => Point2D { x: -1, y: 0 },
        b'v' => Point2D { x: 0, y: 1 },
        b'>' => Point2D { x: 1, y: 0 },
        _ => unreachable!(),
    }
}

// Since it's static once created, the Rc will let us avoid cloning the actual
// path list, which would be expensive relative to the runtime
pub type PathCache = FxHashMap<(Point2D<i8>, Point2D<i8>), Rc<Vec<Vec<u8>>>>;

#[derive(Debug, Clone)]
pub struct KeypadConundrum {
    p1: usize,
    p2: usize,
}

impl FromStr for KeypadConundrum {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut cache = FxHashMap::with_capacity_and_hasher(1000, rustc_hash::FxBuildHasher);
        let mut digit_cache = FxHashMap::with_capacity_and_hasher(1000, rustc_hash::FxBuildHasher);
        let mut nav_cache = FxHashMap::with_capacity_and_hasher(1000, rustc_hash::FxBuildHasher);

        let mut p1 = 0;
        let mut p2 = 0;

        for line in s.trim().lines() {
            let val: usize = line[..3].parse()?;
            p1 += val
                * min_path(
                    line.as_bytes(),
                    0,
                    2,
                    &mut cache,
                    &mut digit_cache,
                    &mut nav_cache,
                );
            p2 += val
                * min_path(
                    line.as_bytes(),
                    0,
                    25,
                    &mut cache,
                    &mut digit_cache,
                    &mut nav_cache,
                );
        }

        Ok(Self { p1, p2 })
    }
}

impl Problem for KeypadConundrum {
    const DAY: usize = 21;
    const TITLE: &'static str = "keypad conundrum";
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

fn digit_paths(from: Point2D<i8>, to: Point2D<i8>, cache: &mut PathCache) -> Rc<Vec<Vec<u8>>> {
    if let Some(cached) = cache.get(&(from, to)) {
        return cached.clone();
    }

    let slope = to - from;

    let mut out = match slope.y.signum() {
        -1 => vec![b'^'; slope.y.unsigned_abs() as usize],
        1 => vec![b'v'; slope.y as usize],
        0 => vec![],
        _ => unreachable!(),
    };

    match slope.x.signum() {
        -1 => out.extend(vec![b'<'; slope.x.unsigned_abs() as usize]),
        1 => out.extend(vec![b'>'; slope.x as usize]),
        _ => {}
    }

    let len = out.len();

    let mut filtered: Vec<Vec<u8>> = out
        .into_iter()
        .permutations(len)
        .unique()
        .filter(|perm| {
            let mut cur = from;
            for ch in perm.iter() {
                cur += direction(*ch);
                if cur == DIGIT_HOLE {
                    return false;
                }
            }
            true
        })
        .map(|mut p| {
            p.push(b'A');
            p
        })
        .collect();

    if filtered.is_empty() {
        filtered.push(vec![b'A']);
    }

    let referenced = Rc::new(filtered);

    cache.insert((from, to), referenced.clone());

    referenced
}

fn nav_paths(from: Point2D<i8>, to: Point2D<i8>, cache: &mut PathCache) -> Rc<Vec<Vec<u8>>> {
    if let Some(cached) = cache.get(&(from, to)) {
        return cached.clone();
    }

    let slope = to - from;

    let mut out = match slope.y.signum() {
        -1 => vec![b'^'; slope.y.unsigned_abs() as usize],
        1 => vec![b'v'; slope.y as usize],
        0 => vec![],
        _ => unreachable!(),
    };

    match slope.x.signum() {
        -1 => out.extend(vec![b'<'; slope.x.unsigned_abs() as usize]),
        1 => out.extend(vec![b'>'; slope.x as usize]),
        _ => {}
    }

    let len = out.len();

    let mut filtered: Vec<Vec<u8>> = out
        .into_iter()
        .permutations(len)
        .unique()
        .filter(|perm| {
            let mut cur = from;
            for ch in perm.iter() {
                cur += direction(*ch);
                if cur == NAV_HOLE {
                    return false;
                }
            }
            true
        })
        .map(|mut p| {
            p.push(b'A');
            p
        })
        .collect();

    if filtered.is_empty() {
        filtered.push(vec![b'A']);
    }

    let referenced = Rc::new(filtered);

    cache.insert((from, to), referenced.clone());

    referenced
}

fn min_path(
    seq: &[u8],
    depth: u8,
    max_depth: u8,
    cache: &mut FxHashMap<(u64, u8, u8), usize>,
    digit_cache: &mut PathCache,
    nav_cache: &mut PathCache,
) -> usize {
    let key = (xxh3_64(seq), depth, max_depth);

    if let Some(prev) = cache.get(&key) {
        return *prev;
    }

    let mut len = 0;
    if depth == 0 {
        let mut cur = digit_pos(b'A');
        for ch in seq {
            let next = digit_pos(*ch);
            let paths = digit_paths(cur, next, digit_cache);
            len += paths
                .iter()
                .map(|p| min_path(p, depth + 1, max_depth, cache, digit_cache, nav_cache))
                .min()
                .unwrap_or_default();

            cur = next;
        }
    } else {
        let mut cur = nav_pos(b'A');
        for ch in seq {
            let next = nav_pos(*ch);
            let paths = nav_paths(cur, next, nav_cache);
            if depth == max_depth {
                len += paths[0].len().max(1);
            } else {
                len += paths
                    .iter()
                    .map(|p| min_path(p, depth + 1, max_depth, cache, digit_cache, nav_cache))
                    .min()
                    .unwrap_or_default();
            }

            cur = next;
        }
    }

    cache.insert(key, len);

    len
}

#[cfg(test)]
mod tests {
    use aoc_plumbing::Solution;

    use super::*;

    #[test]
    #[ignore]
    fn full_dataset() {
        let input = std::fs::read_to_string("input.txt").expect("Unable to load input");
        let solution = KeypadConundrum::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(197560, 242337182910752));
    }

    #[test]
    fn example() {
        let input = "029A
980A
179A
456A
379A";
        let solution = KeypadConundrum::solve(input).unwrap();
        assert_eq!(solution, Solution::new(126384, 154115708116294));
    }
}
