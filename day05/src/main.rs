use itertools::iproduct;
use std::collections::HashMap;
use std::{cmp, fs};
use text_io::scan;

fn solve_1(points: &Vec<(u32, u32, u32, u32)>) {
    let blaa = points
        .iter()
        .filter(|p| p.0 == p.2 || p.1 == p.3)
        .collect::<Vec<&(u32, u32, u32, u32)>>();
    let mut diagram = HashMap::<(u32, u32), u32>::new();
    blaa.iter().for_each(|tup| {
        for (i, j) in iproduct![
            cmp::min(tup.0, tup.2)..=cmp::max(tup.0, tup.2),
            cmp::min(tup.1, tup.3)..=cmp::max(tup.1, tup.3)
        ] {
            let count = diagram.entry((i, j)).or_insert(0);
            *count += 1;
        }
    });
    println!("{:?}", diagram.iter().filter(|(_, val)| val > &&1).count());
}

fn solve_2(points: &Vec<(u32, u32, u32, u32)>) {
    let mut diagram = HashMap::<(u32, u32), u32>::new();
    let mut process = |(i, j)| -> () {
        let count = diagram.entry((i, j)).or_insert(0);
        *count += 1;
    };
    points
        .iter()
        .for_each(|tup| match tup.0 == tup.2 || tup.1 == tup.3 {
            true => {
                for (i, j) in iproduct![
                    cmp::min(tup.0, tup.2)..=cmp::max(tup.0, tup.2),
                    cmp::min(tup.1, tup.3)..=cmp::max(tup.1, tup.3)
                ] {
                    process((i, j));
                }
            }
            false => match (tup.0 > tup.2, tup.1 > tup.3) {
                (true, true) => (tup.2..=tup.0).zip(tup.3..=tup.1).for_each(|p| process(p)),
                (true, false) => (tup.2..=tup.0)
                    .zip((tup.1..=tup.3).rev())
                    .for_each(|p| process(p)),
                (false, true) => ((tup.0..=tup.2).rev())
                    .zip(tup.3..=tup.1)
                    .for_each(|p| process(p)),
                (false, false) => (tup.0..=tup.2).zip(tup.1..=tup.3).for_each(|p| process(p)),
            },
        });
    println!("{:?}", diagram.iter().filter(|(_, val)| val > &&1).count());
}

fn main() {
    let buf = fs::read_to_string("./input.txt").unwrap();
    let input = buf
        .split('\n')
        .map(|line| {
            let (x1, y1, x2, y2): (u32, u32, u32, u32);
            scan!(line.bytes() => "{},{} -> {},{}",x1,y1,x2,y2);
            (x1, y1, x2, y2)
        })
        .collect::<Vec<(u32, u32, u32, u32)>>();
    solve_1(&input);
    solve_2(&input);
}
