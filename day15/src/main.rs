use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};
use std::fs;

#[derive(Debug, Eq, PartialEq)]
struct Vertex {
    cost: u32,
    position: (usize, usize),
    from_v: (usize, usize),
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
        from_v: (0, 0),
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

    /* Code in comments for printing route.
     */
    // let mut route = vec![vec![(0, 0); graph_limit + 1]; graph_limit + 1];
    // let mut print_route = vec![vec![' '; graph_limit + 1]; graph_limit + 1];
    while let Some(vertex) = queue.pop() {
        if !settled.contains(&vertex.position) {
            // route[vertex.position.0][vertex.position.1] = vertex.from_v;
            if vertex.position == (graph_limit, graph_limit) {
                println!("Total Cost {:?}", vertex.cost);
                // let mut pos = (graph_limit, graph_limit);
                // while pos != (0, 0) {
                //     print_route[pos.0][pos.1] = '█';
                //     pos = route[pos.0][pos.1];
                // }
                // print_route
                //     .iter()
                //     .enumerate()
                //     .for_each(|row| println!("{:?} {:?}", row.0, row.1.iter().collect::<String>()));
                break;
            }
            settled.insert(vertex.position.clone());
            get_edges(vertex.position).iter().for_each(|edge| {
                queue.push(Vertex {
                    cost: vertex.cost + get_edge_cost(*edge, is_5x),
                    position: *edge,
                    from_v: vertex.position,
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
