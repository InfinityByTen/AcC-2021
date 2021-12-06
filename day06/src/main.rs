use std::{collections::HashMap, fs};

fn solve_proper(input: &Vec<i32>) {
    // obtained by running simulation for each of the values using solving_stupid method.
    // Sorry, didn't have the energy to work out algebraic solution.
    let blaa = HashMap::<i32, usize>::from([
        (1, 6206821033),
        (2, 5617089148),
        (3, 5217223242),
        (4, 4726100874),
        (5, 4368232009),
    ]);
    let count = input.iter().fold(0_usize, |acc, val| acc + blaa[val]);
    println!("{:?}", count);
}

fn solve_stupid(input: &Vec<i32>) {
    let mut working = input.clone();
    (0..18).for_each(|_day| {
        let mut spawned = 0;
        working.iter_mut().for_each(|e| {
            *e -= 1;
            if e < &mut 0 {
                *e = 6;
                spawned += 1;
            }
        });
        (0..spawned).for_each(|_| working.push(8));
        // println!("{:?}", working);
    });
    println!("{:?}", working.len());
}

fn main() {
    let buf = fs::read_to_string("./input.txt").unwrap();
    let input = buf.split(',').map(|x| x.parse::<i32>().unwrap()).collect();
    solve_stupid(&input);
    solve_proper(&input);
}
