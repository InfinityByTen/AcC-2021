use std::{cmp::min, fs};

fn solve(input: &Vec<usize>, is_increasing: bool) {
    let mut min_count = usize::MAX;
    (*input.iter().min().unwrap()..=*input.iter().max().unwrap()).for_each(|dest| {
        let pass = input.iter().fold(0, |acc, val| {
            let comp = (*val as isize - dest as isize).abs();
            let consumption = match is_increasing {
                true => ((comp * (comp + 1)) / 2),
                false => comp,
            };
            acc + consumption
        }) as usize;
        min_count = min(pass, min_count);
    });
    println!("{:?}", min_count);
}

fn main() {
    let buf = fs::read_to_string("./input.txt").unwrap();
    let input = buf
        .split(',')
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    solve(&input, false);
    solve(&input, true);
}
