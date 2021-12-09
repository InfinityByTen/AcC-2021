use std::collections::{HashMap, HashSet};
use std::fs;

fn solve_1(input: &Vec<Vec<String>>) {
    let (m, n) = (input.len(), input[0].len());
    let loc_map = input
        .iter()
        .enumerate()
        .flat_map(|e| {
            e.1.iter()
                .enumerate()
                .map(move |b| ((e.0 as isize, b.0 as isize), b.1))
        })
        .collect::<HashMap<(isize, isize), &String>>();

    let mut low_points = HashSet::<(isize, isize)>::new();
    let res = (0..m).fold(0, |acc_out, row| {
        acc_out
            + (0..n).fold(0, |acc, col| {
                acc + {
                    let options = vec![
                        (row as isize - 1, col as isize),
                        (row as isize + 1, col as isize),
                        (row as isize, col as isize + 1),
                        (row as isize, col as isize - 1),
                    ];
                    if options.iter().all(|opt| {
                        loc_map.get(opt).is_none()
                            || (input[row][col].parse::<u16>().unwrap()
                                < (**loc_map.get(opt).unwrap()).parse::<u16>().unwrap())
                    }) {
                        low_points.insert((row as isize, col as isize));
                        input[row][col].parse::<u16>().unwrap() + 1
                    } else {
                        0
                    }
                }
            })
    });
    println!("{:?}", res);

    let mut basin_set = HashSet::<(isize, isize)>::new();
    (0..m).for_each(|row| {
        (0..n).for_each(|col| {
            let options = vec![
                (row as isize - 1, col as isize),
                (row as isize + 1, col as isize),
                (row as isize, col as isize + 1),
                (row as isize, col as isize - 1),
            ];
            options
                .iter()
                .filter(|opt| {
                    loc_map.get(opt).is_some()
                        && (input[row][col].parse::<u16>().unwrap()
                            < (**loc_map.get(opt).unwrap()).parse::<u16>().unwrap()
                            && loc_map.get(opt).unwrap() != &"9")
                })
                .for_each(|(i, j)| {
                    basin_set.insert((*i, *j));
                    basin_set.insert((row as isize, col as isize));
                });
        })
    });

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
        for b in basins.iter_mut() {
            let mut dummy = HashSet::new();
            for lp in b.iter() {
                basin_set
                    .iter()
                    .filter(|pt| check_nbd(**pt, **lp))
                    .for_each(|e| {
                        dummy.insert(e);
                    });
            }
            dummy.iter().for_each(|d| {
                b.insert(d);
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
    println!("{:?}", sizes[0] * sizes[1] * sizes[2]);
}

fn main() {
    let buf = fs::read_to_string("./input.txt").unwrap();
    let input = buf
        .split('\n')
        .map(|s| s.chars().map(|c| c.to_string()).collect::<Vec<String>>())
        .collect::<Vec<Vec<String>>>();
    solve_1(&input);
}
