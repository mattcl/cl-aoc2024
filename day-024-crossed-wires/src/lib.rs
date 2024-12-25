use std::{collections::hash_map::Entry, str::FromStr};

use anyhow::{anyhow, bail};
use aoc_plumbing::Problem;
use itertools::Itertools;
use nom::{
    bytes::complete::{tag, take_till},
    character::complete,
    combinator,
    sequence::{separated_pair, terminated, tuple},
    IResult,
};
use rustc_hash::{FxHashMap, FxHashSet};

// This was... not that much fun
#[derive(Debug, Clone)]
pub struct CrossedWires {
    p1: u64,
    p2: String,
}

impl FromStr for CrossedWires {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut arena = GateArena::default();

        let mut initial = true;
        for line in s.trim().lines() {
            if line.is_empty() {
                initial = false;
                continue;
            }

            let (_, parsed) = if initial {
                parse_initial(line).map_err(|e| e.to_owned())?
            } else {
                parse_gate(line).map_err(|e| e.to_owned())?
            };

            arena.insert(parsed);
        }

        arena.prepare();

        let mut p1 = 0;

        for (_, idx) in arena.z_bits.iter().rev() {
            p1 <<= 1;
            if arena.wires[*idx].evaluate(&arena) {
                p1 |= 1;
            }
        }

        // It's a ripple carry adder
        //
        // Bits 0 and the last bit will be half-adders
        //
        // X1 XOR Y1 -> M1    IS
        // X1 AND Y1 -> N1    IC
        // C0 AND M1 -> R1    CIS
        // C0 XOR M1 -> Z1    SUM
        // R1 OR  N1 -> C1    FC

        // okay, we have to find the swapped gates
        let mut suspicious: FxHashSet<&Wire> = FxHashSet::default();

        let origin_candidates: Vec<_> = arena
            .wires
            .iter()
            .filter(|w| w.is_origin() && matches!(w.value, WireValue::Xor { .. }))
            .collect();

        for c in origin_candidates.iter() {
            if let WireValue::Xor {
                left_name,
                right_name,
                ..
            } = c.value
            {
                if left_name.starts_with("x00") || right_name.starts_with("x00") {
                    if !c.name.starts_with("z00") {
                        suspicious.insert(*c);
                    }
                    continue;
                } else if c.name.starts_with("z00") {
                    suspicious.insert(*c);
                    continue;
                }
            }

            if c.name.starts_with('z') {
                suspicious.insert(*c);
            }
        }

        let output_candidates: Vec<_> = arena
            .wires
            .iter()
            .filter(|w| !w.is_origin() && matches!(w.value, WireValue::Xor { .. }))
            .collect();

        for c in output_candidates.iter() {
            if !c.name.starts_with('z') {
                suspicious.insert(c);
            }
        }

        for c in arena.wires.iter().filter(|w| w.name.starts_with('z')) {
            if c.name == "z45" {
                if !matches!(c.value, WireValue::Or { .. }) {
                    suspicious.insert(c);
                }
                continue;
            } else if !matches!(c.value, WireValue::Xor { .. }) {
                suspicious.insert(c);
            }
        }

        let mut additional = Vec::default();

        for c in origin_candidates.iter() {
            if suspicious.contains(c) {
                continue;
            }

            if c.name == "z00" {
                continue;
            }

            if output_candidates
                .iter()
                .filter(|o| o.contains_input(c.name))
                .count()
                == 0
            {
                suspicious.insert(*c);
                additional.push(*c);
            }
        }

        for c in additional.iter() {
            let key = format!(
                "z{}",
                match c.value {
                    WireValue::Or { left_name, .. } => &left_name[1..],
                    WireValue::And { left_name, .. } => &left_name[1..],
                    WireValue::Xor { left_name, .. } => &left_name[1..],
                    _ => bail!("invalid_input"),
                }
            );

            let found = *output_candidates
                .iter()
                .find(|o| o.name == key)
                .ok_or_else(|| anyhow!("invalid input"))?;
            let (left_idx, left_name, right_idx, right_name) = match found.value {
                WireValue::Or {
                    left,
                    left_name,
                    right,
                    right_name,
                }
                | WireValue::And {
                    left,
                    left_name,
                    right,
                    right_name,
                }
                | WireValue::Xor {
                    left,
                    left_name,
                    right,
                    right_name,
                } => (left, left_name, right, right_name),
                _ => bail!("invalid input"),
            };

            let or_gate = arena
                .wires
                .iter()
                .find(|g| {
                    matches!(g.value, WireValue::Or { .. })
                        && (left_name == g.name || right_name == g.name)
                })
                .ok_or_else(|| anyhow!("invalid input"))?;

            if or_gate.name == left_name {
                suspicious.insert(&arena.wires[right_idx]);
            } else {
                suspicious.insert(&arena.wires[left_idx]);
            }
        }

        if suspicious.len() != 8 {
            bail!("could not find solution {}", suspicious.len());
        }

        let p2 = suspicious.iter().map(|w| w.name).sorted().join(",");

        Ok(Self { p1, p2 })
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct GateArena<'a> {
    wires: Vec<Wire<'a>>,
    wire_mapping: FxHashMap<&'a str, usize>,
    z_bits: Vec<(&'a str, usize)>,
    x_bits: Vec<(&'a str, usize)>,
    y_bits: Vec<(&'a str, usize)>,
}

impl<'a> GateArena<'a> {
    pub fn insert(&mut self, input: InputEnum<'a>) {
        let (name, idx) = match input {
            InputEnum::Value { wire, value } => {
                let idx = self.wires.len();
                self.wires.push(Wire {
                    name: wire,
                    idx,
                    value: WireValue::Value(value),
                });
                self.wire_mapping.insert(wire, idx);
                (wire, idx)
            }
            InputEnum::Or { left, right, dest } => {
                let left_idx = *self.wire_mapping.entry(left).or_insert_with(|| {
                    let idx = self.wires.len();
                    self.wires.push(Wire::new(left, idx));
                    idx
                });

                let right_idx = *self.wire_mapping.entry(right).or_insert_with(|| {
                    let idx = self.wires.len();
                    self.wires.push(Wire::new(right, idx));
                    idx
                });

                let dest_idx = match self.wire_mapping.entry(dest) {
                    Entry::Occupied(occupied_entry) => {
                        let existing = *occupied_entry.get();
                        self.wires[existing].value = WireValue::Or {
                            left: left_idx,
                            left_name: left,
                            right: right_idx,
                            right_name: right,
                        };
                        existing
                    }
                    Entry::Vacant(vacant_entry) => {
                        let dest_idx = self.wires.len();

                        self.wires.push(Wire {
                            name: dest,
                            idx: dest_idx,
                            value: WireValue::Or {
                                left: left_idx,
                                left_name: left,
                                right: right_idx,
                                right_name: right,
                            },
                        });
                        vacant_entry.insert(dest_idx);
                        dest_idx
                    }
                };
                (dest, dest_idx)
            }
            InputEnum::And { left, right, dest } => {
                let left_idx = *self.wire_mapping.entry(left).or_insert_with(|| {
                    let idx = self.wires.len();
                    self.wires.push(Wire::new(left, idx));
                    idx
                });

                let right_idx = *self.wire_mapping.entry(right).or_insert_with(|| {
                    let idx = self.wires.len();
                    self.wires.push(Wire::new(right, idx));
                    idx
                });

                let dest_idx = match self.wire_mapping.entry(dest) {
                    Entry::Occupied(occupied_entry) => {
                        let existing = *occupied_entry.get();
                        self.wires[existing].value = WireValue::And {
                            left: left_idx,
                            left_name: left,
                            right: right_idx,
                            right_name: right,
                        };
                        existing
                    }
                    Entry::Vacant(vacant_entry) => {
                        let dest_idx = self.wires.len();

                        self.wires.push(Wire {
                            name: dest,
                            idx: dest_idx,
                            value: WireValue::And {
                                left: left_idx,
                                left_name: left,
                                right: right_idx,
                                right_name: right,
                            },
                        });
                        vacant_entry.insert(dest_idx);
                        dest_idx
                    }
                };
                (dest, dest_idx)
            }
            InputEnum::Xor { left, right, dest } => {
                let left_idx = *self.wire_mapping.entry(left).or_insert_with(|| {
                    let idx = self.wires.len();
                    self.wires.push(Wire::new(left, idx));
                    idx
                });

                let right_idx = *self.wire_mapping.entry(right).or_insert_with(|| {
                    let idx = self.wires.len();
                    self.wires.push(Wire::new(right, idx));
                    idx
                });

                let dest_idx = match self.wire_mapping.entry(dest) {
                    Entry::Occupied(occupied_entry) => {
                        let existing = *occupied_entry.get();
                        self.wires[existing].value = WireValue::Xor {
                            left: left_idx,
                            left_name: left,
                            right: right_idx,
                            right_name: right,
                        };
                        existing
                    }
                    Entry::Vacant(vacant_entry) => {
                        let dest_idx = self.wires.len();

                        self.wires.push(Wire {
                            name: dest,
                            idx: dest_idx,
                            value: WireValue::Xor {
                                left: left_idx,
                                left_name: left,
                                right: right_idx,
                                right_name: right,
                            },
                        });
                        vacant_entry.insert(dest_idx);
                        dest_idx
                    }
                };
                (dest, dest_idx)
            }
        };

        if name.starts_with('z') {
            self.z_bits.push((name, idx));
        } else if name.starts_with('x') {
            self.x_bits.push((name, idx));
        } else if name.starts_with('y') {
            self.y_bits.push((name, idx));
        }
    }

    pub fn prepare(&mut self) {
        self.z_bits.sort();
        self.x_bits.sort();
        self.y_bits.sort();
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Wire<'a> {
    name: &'a str,
    idx: usize,
    value: WireValue<'a>,
}

impl<'a> Wire<'a> {
    pub fn new(name: &'a str, idx: usize) -> Self {
        Self {
            name,
            idx,
            value: WireValue::Unknown,
        }
    }

    pub fn evaluate(&self, arena: &GateArena) -> bool {
        match self.value {
            WireValue::Value(v) => v,
            WireValue::Or { left, right, .. } => {
                arena.wires[left].evaluate(arena) || arena.wires[right].evaluate(arena)
            }
            WireValue::And { left, right, .. } => {
                arena.wires[left].evaluate(arena) && arena.wires[right].evaluate(arena)
            }
            WireValue::Xor { left, right, .. } => {
                arena.wires[left].evaluate(arena) ^ arena.wires[right].evaluate(arena)
            }
            // we should be completely reconciled by the time this is done
            WireValue::Unknown => unreachable!(),
        }
    }

    pub fn is_origin(&self) -> bool {
        match self.value {
            WireValue::Or {
                left_name,
                right_name,
                ..
            } => left_name.starts_with('x') || right_name.starts_with('x'),
            WireValue::And {
                left_name,
                right_name,
                ..
            } => left_name.starts_with('x') || right_name.starts_with('x'),
            WireValue::Xor {
                left_name,
                right_name,
                ..
            } => left_name.starts_with('x') || right_name.starts_with('x'),
            _ => false,
        }
    }

    pub fn contains_input(&self, input: &str) -> bool {
        match self.value {
            WireValue::Or {
                left_name,
                right_name,
                ..
            } => left_name == input || right_name == input,
            WireValue::And {
                left_name,
                right_name,
                ..
            } => left_name == input || right_name == input,
            WireValue::Xor {
                left_name,
                right_name,
                ..
            } => left_name == input || right_name == input,
            _ => false,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum WireValue<'a> {
    Value(bool),
    Or {
        left: usize,
        left_name: &'a str,
        right: usize,
        right_name: &'a str,
    },
    And {
        left: usize,
        left_name: &'a str,
        right: usize,
        right_name: &'a str,
    },
    Xor {
        left: usize,
        left_name: &'a str,
        right: usize,
        right_name: &'a str,
    },
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum InputEnum<'a> {
    Value {
        wire: &'a str,
        value: bool,
    },
    Or {
        left: &'a str,
        right: &'a str,
        dest: &'a str,
    },
    And {
        left: &'a str,
        right: &'a str,
        dest: &'a str,
    },
    Xor {
        left: &'a str,
        right: &'a str,
        dest: &'a str,
    },
}

impl<'a> InputEnum<'a> {
    pub fn starts_with(&self, ch: char) -> Option<&'a str> {
        let v = match self {
            InputEnum::Value { wire, .. } => wire,
            InputEnum::Or { dest, .. } => dest,
            InputEnum::And { dest, .. } => dest,
            InputEnum::Xor { dest, .. } => dest,
        };

        if v.starts_with(ch) {
            Some(v)
        } else {
            None
        }
    }
}

fn parse_initial(input: &str) -> IResult<&str, InputEnum<'_>> {
    combinator::map(
        separated_pair(take_till(|ch| ch == ':'), tag(": "), complete::u8),
        |(wire, value)| InputEnum::Value {
            wire,
            value: value == 1,
        },
    )(input)
}

fn parse_gate(input: &str) -> IResult<&str, InputEnum<'_>> {
    combinator::map(
        separated_pair(parse_gate_op, tag(" -> "), complete::alphanumeric1),
        |((left, kind, right), dest)| match kind {
            "OR" => InputEnum::Or { left, right, dest },
            "AND" => InputEnum::And { left, right, dest },
            "XOR" => InputEnum::Xor { left, right, dest },
            _ => unreachable!(),
        },
    )(input)
}

fn parse_gate_op(input: &str) -> IResult<&str, (&str, &str, &str)> {
    tuple((
        terminated(complete::alphanumeric1, complete::char(' ')),
        terminated(complete::alphanumeric1, complete::char(' ')),
        complete::alphanumeric1,
    ))(input)
}

impl Problem for CrossedWires {
    const DAY: usize = 24;
    const TITLE: &'static str = "crossed wires";
    const README: &'static str = include_str!("../README.md");

    type ProblemError = anyhow::Error;
    type P1 = u64;
    type P2 = String;

    fn part_one(&mut self) -> Result<Self::P1, Self::ProblemError> {
        Ok(self.p1)
    }

    fn part_two(&mut self) -> Result<Self::P2, Self::ProblemError> {
        Ok(self.p2.clone())
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
        let solution = CrossedWires::solve(&input).unwrap();
        assert_eq!(
            solution,
            Solution::new(61495910098126, "css,cwt,gdd,jmv,pqt,z05,z09,z37".into())
        );
    }

    // #[test]
    // fn example() {
    //     let input = "x00: 1
    // x01: 1
    // x02: 1
    // y00: 0
    // y01: 1
    // y02: 0

    // x00 AND y00 -> z00
    // x01 XOR y01 -> z01
    // x02 OR y02 -> z02";
    //     let solution = CrossedWires::solve(input).unwrap();
    //     assert_eq!(solution, Solution::new(4, "".into()));
    // }
}
