use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};
use std::fs;

#[derive(Debug, Eq, PartialEq)]
struct Vertex {
    cost: u32,
    position: (usize, usize),
}

impl Ord for Vertex {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Vertex {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/*
**********************************
Initial approach of doing a sweep.
**********************************
This works for the test, but not the actual input. Most likely there is an upwards or leftward
traversal that I should account for in the main input. It's easy to construct an example
that forces the route around a bunch of 9's using all 1's.

Funnily, this gives an "off by one" error to the answer. Need to then find the bug in it.
Shouldn't be a coincidence. Maybe then there's no upwards or leftwards traversal.
*/
#[allow(dead_code)]
fn solve_attempt_one(input: &Vec<Vec<u32>>, limit: usize) {
    let mut costs = vec![vec![(u32::MAX, (0, 0)); limit + 1]; limit + 1];
    costs[0][0].0 = 0;
    (0..=limit).for_each(|row| {
        (0..=limit).for_each(|col| {
            if row < limit {
                let pot_cost_1 = costs[row][col].0 + input[row + 1][col];
                if costs[row + 1][col].0 > pot_cost_1 {
                    costs[row + 1][col] = (pot_cost_1, (row, col));
                }
            }
            if col < limit {
                let pot_cost_2 = costs[row][col].0 + input[row][col + 1];
                if costs[row][col + 1].0 > pot_cost_2 {
                    costs[row][col + 1] = (pot_cost_2, (row, col));
                }
            }
        })
    });
    costs.iter().for_each(|row| println!("{:?}", row));
    println!("{:?}", costs[limit][limit]);
}

fn solve_1(input: &Vec<Vec<u32>>, limit: usize) {
    let mut queue = BinaryHeap::new();
    queue.push(Vertex {
        cost: 0,
        position: (0, 0),
    });

    let mut settled = HashSet::new();

    let get_edges = |(row, col)| {
        let mut edges = Vec::new();
        if row > 1 {
            edges.push((row - 1, col));
        }
        if row < limit {
            edges.push((row + 1, col));
        }
        if col > 1 {
            edges.push((row, col - 1));
        }
        if col < limit {
            edges.push((row, col + 1));
        }
        edges
    };

    while let Some(vertex) = queue.pop() {
        if !settled.contains(&vertex.position) {
            if vertex.position == (limit, limit) {
                println!("Total Cost {:?}", vertex.cost);
                break;
            }
            settled.insert(vertex.position.clone());
            for edge in get_edges(vertex.position) {
                queue.push(Vertex {
                    cost: vertex.cost + input[edge.0][edge.1],
                    position: edge,
                });
            }
        }
    }
}

fn solve_2(input: &Vec<Vec<u32>>, limit: usize) {
    let mut queue = BinaryHeap::new();
    queue.push(Vertex {
        cost: 0,
        position: (0, 0),
    });

    let mut settled = HashSet::new();

    let get_edges = |(row, col)| {
        let mut edges = Vec::new();
        if row > 1 {
            edges.push((row - 1, col));
        }
        if row < 5 * limit + 4 {
            edges.push((row + 1, col));
        }
        if col > 1 {
            edges.push((row, col - 1));
        }
        if col < 5 * limit + 4 {
            edges.push((row, col + 1));
        }
        edges
    };

    while let Some(vertex) = queue.pop() {
        if !settled.contains(&vertex.position) {
            if vertex.position == (limit * 5 + 4, limit * 5 + 4) {
                println!("Total Cost {:?}", vertex.cost);
                break;
            }
            settled.insert(vertex.position.clone());
            for edge in get_edges(vertex.position) {
                let mapped_edge = (edge.0 % (limit + 1), edge.1 % (limit + 1));
                let overflow = (edge.0 / (limit + 1)) + (edge.1 / (limit + 1));
                let mut val = input[mapped_edge.0][mapped_edge.1] + (overflow as u32);
                if val > 9 {
                    val -= 9;
                }
                queue.push(Vertex {
                    cost: vertex.cost + val,
                    position: edge,
                });
            }
        }
    }
}
fn main() {
    let buf = fs::read_to_string("./input.txt").unwrap();
    let input = buf
        .split('\n')
        .map(|row| {
            row.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();
    solve_1(&input, 99);
    solve_2(&input, 99);
}
