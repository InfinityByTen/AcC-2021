use itertools::Itertools;
use std::fs;

fn solve_1((mut one, mut two): (u32, u32)) {
    let mut roll = (1..=100).cycle();
    let pos_after_roll = |count| match count % 10 {
        0 => 10,
        val => val,
    };

    let (mut one_s, mut two_s) = (0, 0);
    let mut count = 0;
    loop {
        one = pos_after_roll(one + (0..3).fold(0, |acc, _| acc + roll.next().unwrap()));
        count += 3;
        one_s += one;
        if one_s >= 1000 {
            break;
        }
        two = pos_after_roll(two + (0..3).fold(0, |acc, _| acc + roll.next().unwrap()));
        count += 3;
        two_s += two;
        if two_s >= 1000 {
            break;
        }
    }
    println!("{:?}", &one_s.min(two_s) * count);
}

fn solve_2(pos: (u32, u32)) {
    println!("{:?}", pos);
}

fn main() {
    let buf = fs::read_to_string("./test.txt").unwrap();
    let input = buf.split('\n').collect_tuple::<(&str, &str)>().unwrap();

    let pos = (
        input.0.split(": ").nth(1).unwrap().parse::<u32>().unwrap(),
        input.1.split(": ").nth(1).unwrap().parse::<u32>().unwrap(),
    );

    solve_1(pos);
    solve_2(pos);
}
