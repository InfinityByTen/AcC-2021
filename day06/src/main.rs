use std::{collections::HashMap, fs};

fn solve_proper(input: &Vec<i32>, days: u32) {
    let mut counters = HashMap::<i32, usize>::new();
    input.iter().for_each(|state| {
        counters.entry(*state).and_modify(|e| *e += 1).or_insert(1);
    });
    (0..=8).for_each(|state| {
        counters.entry(state).or_insert(0);
    });
    let mut working = counters.clone();
    (0..days).for_each(|_day| {
        (0..8).for_each(|state| {
            if state == 0 {
                // println!("Processing 0 {:?}", counters);
                working.entry(8).and_modify(|e| *e = counters[&0]);
                working.entry(6).and_modify(|e| *e = counters[&0]);
                working.entry(0).and_modify(|e| *e = counters[&1]);
            } else if state == 6 {
                working
                    .entry(state)
                    .and_modify(|e| *e += counters[&(state + 1)]);
            } else {
                {
                    working
                        .entry(state)
                        .and_modify(|e| *e = counters[&(state + 1)]);
                }
            }
        });
        counters = working.clone();
    });
    println!("{:?}", counters.values().sum::<usize>());
}

// Solution for part 2 using precomputations for 256 days
fn solve_pre_comp_256(input: &Vec<i32>) {
    // obtained by running simulation for each of the starting values using solving_stupid method.
    // Runs under a few mins with 32 GB memory.
    let hack_map = HashMap::<i32, usize>::from([
        (1, 6206821033),
        (2, 5617089148),
        (3, 5217223242),
        (4, 4726100874),
        (5, 4368232009),
    ]);
    let count = input.iter().fold(0_usize, |acc, val| acc + hack_map[val]);
    println!("{:?}", count);
}

fn solve_stupid(input: &Vec<i32>, days: u32) {
    let mut working = input.clone();
    (0..days).for_each(|_day| {
        let mut spawned = 0;
        working.iter_mut().for_each(|e| {
            *e -= 1;
            if e < &mut 0 {
                *e = 6;
                spawned += 1;
            }
        });
        (0..spawned).for_each(|_| working.push(8));
    });
    println!("{:?}", working.len());
}

fn main() {
    let buf = fs::read_to_string("./test.txt").unwrap();
    let input = buf.split(',').map(|x| x.parse::<i32>().unwrap()).collect();
    solve_stupid(&input, 80);
    solve_pre_comp_256(&input);
    solve_proper(&input, 256);
}
