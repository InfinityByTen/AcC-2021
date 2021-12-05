use itertools::iproduct;
use std::collections::HashMap;
use std::{cmp::*, fs};
use text_io::scan;

fn process(points: impl Iterator<Item = (u32, u32)>, diagram: &mut HashMap<(u32, u32), u32>) {
    points.for_each(|pt| {
        diagram.entry(pt).and_modify(|e| *e += 1).or_insert(0);
    });
}

fn solve(points: impl Iterator<Item = (u32, u32, u32, u32)>, use_diagonal: bool) {
    let working = points.filter(|p| (p.0 == p.2 || p.1 == p.3) || use_diagonal);
    let mut diagram = HashMap::<(u32, u32), u32>::new();
    working.for_each(
        |tup| match (tup.0 == tup.2 || tup.1 == tup.3, use_diagonal) {
            (true, _) => process(
                iproduct![
                    min(tup.0, tup.2)..=max(tup.0, tup.2),
                    min(tup.1, tup.3)..=max(tup.1, tup.3)
                ],
                &mut diagram,
            ),
            (false, true) => match (tup.0 > tup.2, tup.1 > tup.3) {
                (true, true) => process((tup.2..=tup.0).zip(tup.3..=tup.1), &mut diagram),
                (true, false) => process((tup.2..=tup.0).zip((tup.1..=tup.3).rev()), &mut diagram),
                (false, true) => process(((tup.0..=tup.2).rev()).zip(tup.3..=tup.1), &mut diagram),
                (false, false) => process((tup.0..=tup.2).zip(tup.1..=tup.3), &mut diagram),
            },
            _ => (),
        },
    );
    println!("{:?}", diagram.iter().filter(|(_, val)| val > &&1).count());
}

fn main() {
    let buf = fs::read_to_string("./input.txt").unwrap();
    let input = buf.split('\n').map(|line| {
        let (x1, y1, x2, y2): (u32, u32, u32, u32);
        scan!(line.bytes() => "{},{} -> {},{}",x1,y1,x2,y2);
        (x1, y1, x2, y2)
    });
    solve(input.clone(), false);
    solve(input, true);
}
