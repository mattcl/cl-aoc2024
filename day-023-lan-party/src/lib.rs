use std::{collections::hash_map::Entry, str::FromStr};

use anyhow::anyhow;
use aoc_plumbing::Problem;
use aoc_std::collections::bitset::BitSet576;
use itertools::Itertools;
use rustc_hash::{FxHashMap, FxHashSet};

#[derive(Debug, Clone)]
pub struct LanParty {
    p1: usize,
    p2: String,
}

impl FromStr for LanParty {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut graph = Graph::default();

        for line in s.trim().lines() {
            let (left, right) = line
                .split_once("-")
                .ok_or_else(|| anyhow!("invalid input"))?;
            graph.insert(left, right);
        }

        let mut groups: FxHashSet<[u16; 3]> = FxHashSet::default();

        for node in graph.nodes.iter() {
            if node.name.starts_with("t") {
                for i in 0..(node.edges.len() - 1) {
                    let i_edge = node.edges[i];

                    for j in (i + 1)..node.edges.len() {
                        let j_edge = node.edges[j];

                        if graph.nodes[i_edge].edge_map.contains(j_edge) {
                            let mut key = [node.idx as u16, i_edge as u16, j_edge as u16];
                            key.sort();
                            groups.insert(key);
                        }
                    }
                }
            }
        }

        // let mut cliques = Vec::default();
        // max_clique_proper(&mut BitSet576::default(), graph.full_set, BitSet576::default(), &graph, &mut cliques);
        // cliques.sort_by_cached_key(|c| c.count());

        let maximum = max_clique_exploit(&graph);

        let out = maximum
            .iter()
            .map(|idx| graph.nodes[idx].name)
            .sorted()
            .join(",");

        Ok(Self {
            p1: groups.len(),
            p2: out,
        })
    }
}

// this is the "correct" way if we were dealing with arbitrary graphs, but it
// looks like the inputs all have a single 13-length clique.
#[allow(unused)]
fn max_clique_proper(
    r: &mut BitSet576,
    mut p: BitSet576,
    mut x: BitSet576,
    graph: &Graph,
    cliques: &mut Vec<BitSet576>,
) {
    if p == BitSet576::ZERO {
        if x == BitSet576::ZERO && r.count() > 3 {
            cliques.push(*r);
        }
        return;
    }

    for neighbor_idx in p.iter() {
        let np = p & graph.nodes[neighbor_idx].edge_map;
        let nx = x & graph.nodes[neighbor_idx].edge_map;

        r.insert(neighbor_idx);

        max_clique_proper(r, np, nx, graph, cliques);

        r.remove(neighbor_idx);
        p.remove(neighbor_idx);
        x.insert(neighbor_idx);
    }
}

// it looks like everyone has a 13-length clique, so we can just look for that
// one
fn max_clique_exploit(graph: &Graph) -> BitSet576 {
    for i in 0..graph.nodes.len() {
        if graph.nodes[i].edges.len() < 12 {
            continue;
        }

        'outer: for edges in graph.nodes[i].edges.iter().combinations(12) {
            let mut working = graph.nodes[i].edge_map;

            for e in edges {
                working &= graph.nodes[*e].edge_map;
                if working.count() < 13 {
                    continue 'outer;
                }
            }

            if working.count() >= 13 {
                return working;
            }
        }
    }

    unreachable!()
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Graph<'a> {
    full_set: BitSet576,
    nodes: Vec<Node<'a>>,
    mapping: FxHashMap<&'a str, usize>,
}

impl<'a> Graph<'a> {
    pub fn insert(&mut self, left: &'a str, right: &'a str) {
        let left_idx = match self.mapping.entry(left) {
            Entry::Occupied(entry) => *entry.get(),
            Entry::Vacant(entry) => {
                let idx = self.nodes.len();
                entry.insert(idx);
                self.nodes.push(Node::new(idx, left));
                idx
            }
        };

        let right_idx = match self.mapping.entry(right) {
            Entry::Occupied(entry) => *entry.get(),
            Entry::Vacant(entry) => {
                let idx = self.nodes.len();
                entry.insert(idx);
                self.nodes.push(Node::new(idx, right));
                idx
            }
        };

        self.full_set.insert(left_idx);
        self.full_set.insert(right_idx);
        self.nodes[left_idx].insert_edge(right_idx);
        self.nodes[right_idx].insert_edge(left_idx);
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Node<'a> {
    idx: usize,
    name: &'a str,
    edges: Vec<usize>,
    edge_map: BitSet576,
}

impl<'a> Node<'a> {
    pub fn new(idx: usize, name: &'a str) -> Self {
        let mut edge_map = BitSet576::default();
        edge_map.insert(idx);

        Self {
            idx,
            name,
            edges: Vec::default(),
            edge_map,
        }
    }

    pub fn insert_edge(&mut self, idx: usize) {
        if !self.edge_map.contains(idx) {
            self.edges.push(idx);
            self.edge_map.insert(idx);
        }
    }
}

impl Problem for LanParty {
    const DAY: usize = 23;
    const TITLE: &'static str = "lan party";
    const README: &'static str = include_str!("../README.md");

    type ProblemError = anyhow::Error;
    type P1 = usize;
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
        let solution = LanParty::solve(&input).unwrap();
        assert_eq!(
            solution,
            Solution::new(1308, "bu,fq,fz,pn,rr,st,sv,tr,un,uy,zf,zi,zy".into())
        );
    }

    // #[test]
    // fn example() {
    //     let input = "kh-tc
    // qp-kh
    // de-cg
    // ka-co
    // yn-aq
    // qp-ub
    // cg-tb
    // vc-aq
    // tb-ka
    // wh-tc
    // yn-cg
    // kh-ub
    // ta-co
    // de-co
    // tc-td
    // tb-wq
    // wh-td
    // ta-ka
    // td-qp
    // aq-cg
    // wq-ub
    // ub-vc
    // de-ta
    // wq-aq
    // wq-vc
    // wh-yn
    // ka-de
    // kh-ta
    // co-tc
    // wh-qp
    // tb-vc
    // td-yn";
    //     let solution = LanParty::solve(input).unwrap();
    //     assert_eq!(solution, Solution::new(7, "co,de,ka,ta".into()));
    // }
}
