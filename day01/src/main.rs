use std::fs;

fn solve_1(depths: &Vec<u32>) {
    let count = depths.windows(2).filter(|tup| tup[1] > tup[0]).count();
    println!("{:?}", count);
}

fn solve_2(depths: &Vec<u32>) {
    let count = depths
        .windows(3)
        .map(|win| win.iter().sum())
        .collect::<Vec<u32>>() // this assignment is not nice
        .windows(2)
        .filter(|tup| tup[1] > tup[0])
        .count();
    println!("{:?}", count);
}

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let depths = input
        .split('\n')
        .map(|x| x.parse::<u32>().unwrap())
        .collect();
    solve_1(&depths);
    solve_2(&depths);
}
