use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};
use std::fs;

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

fn run_dijkstra(input: &Vec<Vec<u32>>, is_5x: bool) {
    let limit = input.len() - 1;
    let graph_limit = match is_5x {
        true => 5 * input.len() - 1,
        false => limit,
    };
    let mut queue = BinaryHeap::new();
    queue.push(Vertex {
        cost: 0,
        position: (0, 0),
    });

    let mut settled = HashSet::new();
    let get_edges = |(row, col)| -> _ {
        vec![
            (row + 1, col),
            (row - 1, col),
            (row, col + 1),
            (row, col - 1),
        ]
        .iter()
        .cloned()
        .filter(|(i, j)| (0..=graph_limit).contains(i) && (0..=graph_limit).contains(j))
        .collect::<Vec<(usize, usize)>>()
    };

    let get_edge_cost = |edge: (usize, usize), is_5x: bool| match is_5x {
        true => {
            let mapped_edge = (edge.0 % (limit + 1), edge.1 % (limit + 1));
            let overflow = (edge.0 / (limit + 1)) + (edge.1 / (limit + 1));
            let mut val = input[mapped_edge.0][mapped_edge.1] + (overflow as u32);
            if val > 9 {
                val -= 9;
            }
            val
        }
        false => input[edge.0][edge.1],
    };

    while let Some(vertex) = queue.pop() {
        if !settled.contains(&vertex.position) {
            if vertex.position == (graph_limit, graph_limit) {
                println!("Total Cost {:?}", vertex.cost);
                break;
            }
            settled.insert(vertex.position.clone());
            get_edges(vertex.position).iter().for_each(|edge| {
                queue.push(Vertex {
                    cost: vertex.cost + get_edge_cost(*edge, is_5x),
                    position: *edge,
                });
            })
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
    run_dijkstra(&input, false);
    run_dijkstra(&input, true);
}
