use itertools::iproduct;
use std::collections::HashSet;
use std::fs;

fn print_oct(input: &Vec<Vec<u32>>) {
    input
        .iter()
        .for_each(|l| println!("{:?}", l.iter().map(|v| v.to_string()).collect::<String>()));
}

fn process_flashes(
    input: &mut Vec<Vec<u32>>,
    prev_flashes: &HashSet<(usize, usize)>,
    to_process: &HashSet<(usize, usize)>,
) -> usize {
    let mut flash_count = 0;
    let adj = |row: isize, col: isize| {
        iproduct![row - 1..=row + 1, col - 1..=col + 1].filter(|(i, j)| {
            (0..10).contains(i)
                && (0..10).contains(j)
                && !to_process.contains(&(*i as usize, *j as usize))
                && !prev_flashes.contains(&(*i as usize, *j as usize))
        })
    };

    let mut new_flashes = HashSet::new();
    to_process.iter().for_each(|flash| {
        adj(flash.0 as isize, flash.1 as isize).for_each(|adj| {
            input[adj.0 as usize][adj.1 as usize] += 1;
            if input[adj.0 as usize][adj.1 as usize] >= 10 {
                new_flashes.insert((adj.0 as usize, adj.1 as usize));
            }
        });
    });
    new_flashes
        .iter()
        .for_each(|(ni, nj)| input[*ni as usize][*nj as usize] = 0);

    if !new_flashes.is_empty() {
        let comb = prev_flashes.union(to_process).cloned().collect();
        flash_count += new_flashes.len() + process_flashes(input, &comb, &new_flashes);
    }
    flash_count
}

fn solve(input: &mut Vec<Vec<u32>>) {
    let mut flash_count = 0;
    let mut step = 0;

    loop {
        let mut flashed = HashSet::new();
        (0..10).for_each(|row| {
            (0..10).for_each(|col| {
                input[row][col] += 1;
                if input[row][col] == 10 {
                    flashed.insert((row, col));
                    input[row][col] = 0;
                }
            })
        });
        flash_count += flashed.len() + process_flashes(input, &HashSet::new(), &flashed);

        step += 1;
        if step == 100 {
            print_oct(&input);
            println!("{:?}", flash_count);
        }
        if input.iter().flat_map(|x| x).all(|v| v == &0) {
            println!("Simultaneous flash at {:?}", step);
            print_oct(&input);
            break;
        }
    }
}

fn main() {
    let buf = fs::read_to_string("./input.txt").unwrap();
    let mut input = buf
        .split('\n')
        .map(|e| {
            e.chars()
                .map(|c| c.to_digit(10_u32).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();
    solve(&mut input);
}
