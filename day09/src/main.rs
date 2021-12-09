use std::collections::{HashMap, HashSet};
use std::fs;

fn solve_1(input: &Vec<Vec<u32>>) {
    let (m, n) = (input.len() as isize, input[0].len() as isize);
    let loc_map = input
        .iter()
        .enumerate()
        .flat_map(|e| {
            e.1.iter()
                .enumerate()
                .map(move |b| ((e.0 as isize, b.0 as isize), b.1))
        })
        .collect::<HashMap<(isize, isize), &u32>>();

    let mut low_points = HashSet::<(isize, isize)>::new();
    let mut basin_set = HashSet::<(isize, isize)>::new();
    let adj = |row, col| -> Vec<(isize, isize)> {
        vec![
            (row - 1, col),
            (row + 1, col),
            (row, col + 1),
            (row, col - 1),
        ]
    };

    let res = (0..m).fold(0, |acc_out, row| {
        acc_out
            + (0..n).fold(0, |acc, col| {
                adj(row, col)
                    .iter()
                    .filter(|opt| {
                        loc_map.get(opt).is_some()
                            && (input[row as usize][col as usize] < (**loc_map.get(opt).unwrap())
                                && loc_map.get(opt).unwrap() != &&9)
                    })
                    .for_each(|(i, j)| {
                        basin_set.insert((*i, *j));
                    });

                acc + {
                    if adj(row, col).iter().all(|opt| {
                        loc_map.get(opt).is_none()
                            || (input[row as usize][col as usize] < (**loc_map.get(opt).unwrap()))
                    }) {
                        low_points.insert((row as isize, col as isize));
                        input[row as usize][col as usize] + 1
                    } else {
                        0
                    }
                }
            })
    });
    println!("{:?}", res);

    let mut basins = low_points
        .iter()
        .map(|pt| HashSet::from([pt]))
        .collect::<Vec<HashSet<_>>>();

    let check_nbd = |a: (isize, isize), b: (isize, isize)| -> bool {
        (((b.0 - 1)..=(b.0 + 1)).contains(&a.0) && (b.1 == a.1))
            || (b.0 == a.0 && ((b.1 - 1)..=(b.1 + 1)).contains(&a.1))
    };

    let mut i = basins.iter().fold(0, |acc, b| acc + b.len());
    loop {
        for basin in basins.iter_mut() {
            let mut temp = HashSet::new();
            for lp in basin.iter() {
                basin_set
                    .iter()
                    .filter(|pt| check_nbd(**pt, **lp))
                    .for_each(|e| {
                        temp.insert(e);
                    });
            }
            temp.iter().for_each(|t| {
                basin.insert(t);
            });
        }
        if basins.iter().fold(0, |acc, b| acc + b.len()) == i {
            break;
        }
        i = basins.iter().fold(0, |acc, b| acc + b.len());
    }
    let mut sizes = basins.iter().map(|b| b.len()).collect::<Vec<usize>>();
    sizes.sort();
    sizes.reverse();
    println!("{:?}", sizes.iter().take(3).product::<usize>());
}

fn main() {
    let buf = fs::read_to_string("./input.txt").unwrap();
    let input = buf
        .split('\n')
        .map(|s| {
            s.chars()
                .map(|c| c.to_string().parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();
    solve_1(&input);
}
