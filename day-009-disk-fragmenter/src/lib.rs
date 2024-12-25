use std::{collections::BTreeSet, str::FromStr};

use aoc_plumbing::Problem;

#[derive(Debug, Clone)]
pub struct DiskFragmenter {
    files: Vec<AocFile>,
    free_buckets: Vec<BTreeSet<usize>>,
}

impl FromStr for DiskFragmenter {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut files = Vec::default();
        let mut free_buckets = vec![BTreeSet::default(); 10];
        let mut pos = 0;
        for (idx, chunk) in s.trim().as_bytes().chunks(2).enumerate() {
            let f = match chunk {
                [size, free] => AocFile {
                    id: idx,
                    pos,
                    size: (size - b'0') as usize,
                    free: (free - b'0') as usize,
                },
                [size] => AocFile {
                    id: idx,
                    pos,
                    size: (size - b'0') as usize,
                    free: 0,
                },
                _ => unreachable!(),
            };
            pos += f.size + f.free;
            if f.free > 0 {
                free_buckets[f.free].insert(f.pos + f.size);
            }
            files.push(f);
        }
        Ok(Self {
            files,
            free_buckets,
        })
    }
}

impl Problem for DiskFragmenter {
    const DAY: usize = 9;
    const TITLE: &'static str = "disk fragmenter";
    const README: &'static str = include_str!("../README.md");

    type ProblemError = anyhow::Error;
    type P1 = usize;
    type P2 = usize;

    fn part_one(&mut self) -> Result<Self::P1, Self::ProblemError> {
        let mut checksum = 0;

        // use this arrangement which is more complicated so that we don't
        // mutate anything before we're ready to do so in part 2
        let mut pos = 0;
        let mut head = 0;
        let mut tail = self.files.len() - 1;
        let mut rear = self.files[tail];
        while tail >= head {
            let cur = if tail == head { rear } else { self.files[head] };

            checksum += cur.checksum(pos);

            pos += cur.size;

            // if we have free space for this file, we need to fill it
            let mut free = cur.free;
            while free > 0 && tail > head {
                let taken = rear.take(free);

                // do this here instead of with the file funciton since we might
                // not take all the blocks
                checksum += rear.id * (pos + pos + taken - 1) * taken / 2;

                pos += taken;
                free -= taken;

                if rear.is_empty() {
                    tail -= 1;
                    rear = self.files[tail];
                }
            }

            head += 1;
        }

        Ok(checksum)
    }

    fn part_two(&mut self) -> Result<Self::P2, Self::ProblemError> {
        let mut checksum = 0;

        for idx in 0..self.files.len() {
            let tail = self.files.len() - 1 - idx;
            let cur = self.files[tail];

            // find the lowest free position that can fit us
            let mut min_bucket = usize::MAX;
            let mut min_pos = usize::MAX;
            for bucket_idx in cur.size..self.free_buckets.len() {
                if !self.free_buckets[bucket_idx].is_empty() {
                    let candidate = self.free_buckets[bucket_idx].first().copied().unwrap();
                    if candidate < min_pos {
                        min_pos = candidate;
                        min_bucket = bucket_idx;
                    }
                }
            }

            if min_bucket < usize::MAX && min_pos < cur.pos {
                // we know this isn't empty because we just checked when
                // finding the min
                let free_pos = self.free_buckets[min_bucket].pop_first().unwrap();

                let rem = min_bucket - cur.size;

                checksum += cur.checksum(free_pos);

                if rem > 0 {
                    self.free_buckets[rem].insert(free_pos + cur.size);
                }
            } else {
                // if we're here, we couldn't move, so calculate the checkum in place
                checksum += cur.checksum(cur.pos);
            }
        }

        Ok(checksum)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AocFile {
    id: usize,
    pos: usize,
    size: usize,
    free: usize,
}

impl AocFile {
    pub fn take(&mut self, amount: usize) -> usize {
        if amount > self.size {
            let taken = self.size;
            self.size = 0;
            taken
        } else {
            self.size -= amount;
            amount
        }
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn checksum(&self, pos: usize) -> usize {
        self.id * (pos + pos + self.size - 1) * self.size / 2
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
        let solution = DiskFragmenter::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(6349606724455, 6376648986651));
    }

    #[test]
    fn example() {
        let input = "2333133121414131402";
        let solution = DiskFragmenter::solve(input).unwrap();
        assert_eq!(solution, Solution::new(1928, 2858));
    }
}
