use std::cmp::Ordering;
use std::{collections::BinaryHeap, str::FromStr};

use aoc_plumbing::Problem;
use aoc_std::{collections::CharGrid, directions::Cardinal, geometry::Location};
use rustc_hash::{FxHashMap, FxHashSet};

#[derive(Debug, Clone)]
pub struct ReindeerMaze {
    p1: usize,
    p2: usize,
}

impl FromStr for ReindeerMaze {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut grid = CharGrid::from_str(s)?;
        let mut graph = Graph::default();

        let start = Location::new(grid.height() - 2, 1);
        let end = Location::new(1, grid.width() - 2);

        // the starting and ending locations
        graph.insert_node(start);
        graph.insert_node(end);
        grid.locations[start.row][start.col] = 'X';
        grid.locations[end.row][end.col] = 'X';

        // let's collapse the grid to just the junctions
        for r in 1..grid.height() - 1 {
            for c in 1..grid.width() - 1 {
                if grid.locations[r][c] == '.' {
                    let loc = Location::new(r, c);
                    if grid
                        .cardinal_neighbors(&loc)
                        .filter(|(_, _, v)| **v == '.')
                        .count()
                        > 2
                    {
                        graph.insert_node(loc);
                        grid.locations[r][c] = 'X';
                    }
                }
            }
        }

        // The idea is to collapse the grid into a graph where we pre-compute
        // the costs between junctions (and the start and end as junctions).
        //
        // This will hopefully make the resulting search faster because it
        // doesn't have to examine as many nodes.

        // bfs all the nodes to their closest neighbors in each direction
        let mut cur = Vec::with_capacity(100);
        let mut next = Vec::with_capacity(100);
        for i in 0..graph.nodes.len() {
            bfs_junction(i, &grid, &mut graph, &mut cur, &mut next);
        }

        // cool, now let's remove always-bad edges, nodes that only have 2 edges
        // that are not the start and end nodes, and let's remove edges that
        // lead to a dead end node.
        //
        // make two passes
        for _ in 0..2 {
            // start at 2 to avoid the start/end nodes
            for i in (2..graph.nodes.len()).rev() {
                // remove dead-end nodes
                if graph.nodes[i].edges.len() < 2 {
                    remove_single_edge_nodes(&mut graph, i);
                    continue;
                }

                // remove multiple paths to same destination, if able
                if graph.nodes[i].edges.len() > 2 {
                    'outer: for j in 0..graph.nodes[i].edges.len() {
                        for k in (j + 1)..graph.nodes[i].edges.len() {
                            if graph.nodes[i].edges[j].to == graph.nodes[i].edges[k].to {
                                let left = graph.nodes[i].edges[j];
                                let right = graph.nodes[i].edges[k];

                                #[allow(clippy::comparison_chain)]
                                if left.cost < right.cost {
                                    graph.nodes[i].edges.remove(k);
                                    graph.nodes[right.to].edges.retain(|e| {
                                        e.to != i || e.exit_dir != right.enter_dir.opposite()
                                    });
                                    break 'outer;
                                } else if right.cost < left.cost {
                                    graph.nodes[i].edges.remove(j);
                                    graph.nodes[left.to].edges.retain(|e| {
                                        e.to != i || e.exit_dir != left.enter_dir.opposite()
                                    });
                                    break 'outer;
                                } else {
                                    graph.nodes[i].edges[j].distance += right.distance;
                                    graph.nodes[i].edges.remove(k);
                                    graph.nodes[right.to].edges.retain(|e| {
                                        e.to != i || e.exit_dir != right.enter_dir.opposite()
                                    });
                                    if let Some(other) =
                                        graph.nodes[right.to].edges.iter_mut().find(|e| {
                                            e.to == i && e.exit_dir == left.enter_dir.opposite()
                                        })
                                    {
                                        other.distance += right.distance;
                                    }
                                    break 'outer;
                                }
                            }
                        }
                    }
                }

                // remove join the edges of nodes that are effectively corridors
                if graph.nodes[i].edges.len() == 2 {
                    let left = graph.nodes[i].edges[0];
                    let right = graph.nodes[i].edges[1];
                    graph.nodes[i].edges.clear();

                    // we want to join these edges, so we need to know how much
                    // it costs to move through it
                    let traverse_cost = if left.enter_dir.opposite() == right.enter_dir {
                        1
                    } else {
                        1001
                    };

                    let cost = left.cost + right.cost + traverse_cost;

                    // the new distance includes the junction we're removing
                    let dist = left.distance + right.distance + 1;

                    for e in graph.nodes[left.to].edges.iter_mut() {
                        if e.to == i {
                            e.to = right.to;
                            e.exit_dir = right.exit_dir;
                            e.distance = dist;
                            e.cost = cost;
                            break;
                        }
                    }

                    for e in graph.nodes[right.to].edges.iter_mut() {
                        if e.to == i {
                            e.to = left.to;
                            e.exit_dir = left.exit_dir;
                            e.distance = dist;
                            e.cost = cost;
                            break;
                        }
                    }
                }
            }
        }

        // we might have situations (or a variant of this)
        //       +-------+
        //       |       |
        // X --- A ----- B ---- Y
        //
        // we can collapse all of these into X -- Y by computing the cheapest
        // cost/distance through the junctions A, B
        for i in 2..graph.nodes.len() {
            if graph.nodes[i].edges.len() == 3 {
                if graph.nodes[i].edges[0].to == graph.nodes[i].edges[1].to {
                    collapse_forked_rejoin(&mut graph, i, 0, 1, 2);
                    continue;
                }

                if graph.nodes[i].edges[0].to == graph.nodes[i].edges[2].to {
                    collapse_forked_rejoin(&mut graph, i, 0, 2, 1);
                    continue;
                }

                if graph.nodes[i].edges[1].to == graph.nodes[i].edges[2].to {
                    collapse_forked_rejoin(&mut graph, i, 1, 2, 0);
                    continue;
                }
            }
        }

        // okay, now we can solve both parts, i guess
        let mut seen_locations = vec![usize::MAX - 2000; graph.nodes.len()];
        seen_locations[0] = 0;
        let min = best(&graph, &mut seen_locations);

        let total_dist = all_paths(&graph, &mut seen_locations, min);

        Ok(Self {
            p1: min,
            p2: total_dist,
        })
    }
}

fn bfs_junction(
    idx: usize,
    grid: &CharGrid,
    graph: &mut Graph,
    cur: &mut Vec<(Node, Cardinal, usize, usize)>,
    next: &mut Vec<(Node, Cardinal, usize, usize)>,
) {
    let start = graph.nodes[idx].location;

    for (d, l, _) in grid
        .cardinal_neighbors(&start)
        .filter(|(_, _, v)| **v == '.')
    {
        cur.push((
            Node {
                location: l,
                facing: d,
            },
            d,
            0,
            0,
        ));
    }

    while !cur.is_empty() {
        for (node, orig_facing, dist, cost) in cur.iter() {
            // we should only ever find one such neighbor
            for (d, l, nv) in grid
                .cardinal_neighbors(&node.location)
                .filter(|(nd, _, nv)| **nv != '#' && nd.opposite() != node.facing)
            {
                let next_cost = cost + if d == node.facing { 1 } else { 1001 };

                if *nv == 'X' && l != start {
                    // make an edge between our parent junction and the one we
                    // just found

                    // this must exist
                    let other = graph.node_map.get(&l).copied().unwrap();

                    graph.nodes[idx].edges.push(Edge {
                        from: idx,
                        to: other,
                        enter_dir: *orig_facing,
                        exit_dir: d,
                        distance: dist + 1,
                        cost: next_cost,
                    });

                    continue;
                }

                next.push((
                    Node {
                        location: l,
                        facing: d,
                    },
                    *orig_facing,
                    dist + 1,
                    next_cost,
                ));
            }
        }

        cur.clear();

        std::mem::swap(cur, next);
    }
}

fn remove_single_edge_nodes(graph: &mut Graph, i: usize) {
    if graph.nodes[i].edges.len() == 1 {
        let to = graph.nodes[i].edges[0].to;
        graph.nodes[to].edges.retain(|e| e.to != i);
        graph.nodes[i].edges.clear();
        remove_single_edge_nodes(graph, to);
    }
}

fn collapse_forked_rejoin(graph: &mut Graph, i: usize, up_i: usize, dn_i: usize, r_i: usize) {
    let other = graph.nodes[i].edges[up_i].to;

    if graph.nodes[other].edges.len() != 3 {
        return;
    }

    if let Some(rem_right) = graph.nodes[other].edges.iter().find(|e| e.to != i).copied() {
        let rem_left = graph.nodes[i].edges[r_i];
        // okay, we're going to join the unique node that we have
        // with the unique node the other has, bypassing both
        // us and the unique node
        let up = graph.nodes[i].edges[up_i];
        let dn = graph.nodes[i].edges[dn_i];

        let cost_up = rem_left.cost
            + rem_right.cost
            + up.cost
            + if rem_left.enter_dir.opposite() == up.enter_dir {
                1
            } else {
                1001
            }
            + if rem_right.enter_dir == up.exit_dir {
                1
            } else {
                1001
            };

        let cost_dn = rem_left.cost
            + rem_right.cost
            + dn.cost
            + if rem_left.enter_dir.opposite() == dn.enter_dir {
                1
            } else {
                1001
            }
            + if rem_right.enter_dir == dn.exit_dir {
                1
            } else {
                1001
            };

        let dist_up = rem_left.distance + rem_right.distance + up.distance + 2;
        let dist_dn = rem_left.distance + rem_right.distance + dn.distance + 2;

        #[allow(clippy::comparison_chain)]
        let (final_cost, final_dist) = if cost_up == cost_dn {
            (cost_up, dist_up + dist_dn - 2)
        } else if cost_up < cost_dn {
            (cost_up, dist_up)
        } else {
            (cost_dn, dist_dn)
        };

        for e in graph.nodes[rem_left.to].edges.iter_mut() {
            if e.to == i {
                e.to = rem_right.to;
                e.exit_dir = rem_right.exit_dir;
                e.distance = final_dist;
                e.cost = final_cost;
                break;
            }
        }

        for e in graph.nodes[rem_right.to].edges.iter_mut() {
            if e.to == other {
                e.to = rem_left.to;
                e.exit_dir = rem_left.exit_dir;
                e.distance = final_dist;
                e.cost = final_cost;
                break;
            }
        }
        // remove ourselves from the valid nodes
        graph.nodes[i].edges.clear();
        graph.nodes[other].edges.clear();
    }
}

fn best(graph: &Graph, seen_locations: &mut [usize]) -> usize {
    let mut heap = BinaryHeap::default();

    let start = SimpleState {
        node: 0,
        facing: Cardinal::East,
        cost: 0,
    };

    heap.push(start);

    while let Some(SimpleState { node, facing, cost }) = heap.pop() {
        if node == 1 {
            return cost;
        }

        if seen_locations[node] < cost {
            continue;
        }

        seen_locations[node] = cost;

        for edge in graph.nodes[node].edges.iter() {
            if edge.enter_dir.opposite() == facing {
                continue;
            }

            let next_cost = cost + edge.cost + if edge.enter_dir == facing { 1 } else { 1001 };

            let next_node = edge.to;

            let next_state = SimpleState {
                node: next_node,
                cost: next_cost,
                facing: edge.exit_dir,
            };

            heap.push(next_state);
        }
    }

    0
}

fn all_paths(graph: &Graph, seen_locations: &mut [usize], min: usize) -> usize {
    let mut heap = BinaryHeap::default();

    let start = State {
        node: 0,
        cost: 0,
        facing: Cardinal::East,
        link: usize::MAX,
    };

    heap.push(start);

    let mut unique: FxHashSet<(usize, usize)> =
        FxHashSet::with_capacity_and_hasher(1000, rustc_hash::FxBuildHasher);
    let mut unique_junctions: Vec<bool> = vec![false; graph.nodes.len()];
    let mut state_links: Vec<StateLink> = Vec::with_capacity(5000);

    let mut total_dist = 0;

    while let Some(State {
        node,
        cost,
        facing,
        link,
    }) = heap.pop()
    {
        if node == 1 {
            if cost > min {
                break;
            }

            let mut cur_link = link;
            while cur_link != usize::MAX {
                let edge = state_links[cur_link].edge;
                if unique.insert(edge.unique_id()) {
                    unique_junctions[edge.from] = true;
                    unique_junctions[edge.to] = true;
                    total_dist += edge.distance;
                }
                cur_link = state_links[cur_link].prev;
            }

            continue;
        }

        for edge in graph.nodes[node].edges.iter() {
            if edge.enter_dir.opposite() == facing {
                continue;
            }

            let next_cost = cost + edge.cost + if edge.enter_dir == facing { 1 } else { 1001 };

            let next_node = edge.to;

            // do this inside, since it's much more costly to have to calculate
            // the next state to put on the heap because of the edge array
            if seen_locations[next_node] + 1000 < next_cost {
                continue;
            }

            if seen_locations[next_node] == usize::MAX - 2000 {
                seen_locations[next_node] = next_cost;
            }

            // we can do this pseudo linked-list using the state_links arena
            // instead of having to maintain a list (and clone that list) for
            // each new state.
            let new_link = StateLink { edge, prev: link };

            let new_link_idx = state_links.len();

            state_links.push(new_link);

            let next_state = State {
                node: next_node,
                cost: next_cost,
                facing: edge.exit_dir,
                link: new_link_idx,
            };

            heap.push(next_state);
        }
    }

    total_dist + unique_junctions.iter().filter(|v| **v).count()
}

#[derive(Debug, Clone, Default)]
pub struct Graph {
    node_map: FxHashMap<Location, usize>,
    nodes: Vec<GraphNode>,
}

impl Graph {
    pub fn insert_node(&mut self, location: Location) {
        let id = self.nodes.len();
        self.nodes.push(GraphNode {
            location,
            ..Default::default()
        });
        self.node_map.insert(location, id);
    }
}

#[derive(Debug, Clone, Default)]
pub struct GraphNode {
    location: Location,
    edges: Vec<Edge>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Edge {
    from: usize,
    to: usize,
    enter_dir: Cardinal,
    exit_dir: Cardinal,
    distance: usize,
    cost: usize,
}

impl Edge {
    pub fn unique_id(&self) -> (usize, usize) {
        (self.from.min(self.to), self.from.max(self.to))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Node {
    location: Location,
    facing: Cardinal,
}

impl Problem for ReindeerMaze {
    const DAY: usize = 16;
    const TITLE: &'static str = "reindeer maze";
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

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct SimpleState {
    node: usize,
    facing: Cardinal,
    cost: usize,
}

impl Ord for SimpleState {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.node.cmp(&other.node))
    }
}

impl PartialOrd for SimpleState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct State {
    node: usize,
    cost: usize,
    facing: Cardinal,
    link: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.node.cmp(&other.node))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StateLink<'a> {
    edge: &'a Edge,
    prev: usize,
}

#[cfg(test)]
mod tests {
    use aoc_plumbing::Solution;

    use super::*;

    #[test]
    #[ignore]
    fn full_dataset() {
        let input = std::fs::read_to_string("input.txt").expect("Unable to load input");
        let solution = ReindeerMaze::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(115500, 679));
    }

    #[test]
    fn example() {
        let input = "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";
        let solution = ReindeerMaze::solve(input).unwrap();
        assert_eq!(solution, Solution::new(7036, 45));
    }
}
