use itertools::iproduct;
use std::cmp::max;
use std::fs;
use text_io::scan;

fn hits_target(start: (i32, i32), (x1, x2, y1, y2): (i32, i32, i32, i32)) -> bool {
    let mut pos = (0, 0);
    let (mut x, mut y) = start;
    loop {
        pos = (pos.0 + x, pos.1 + y);
        if pos.0 >= x1 && pos.0 <= x2 && pos.1 >= y1 && pos.1 <= y2 {
            return true;
        }
        if pos.1 < y1 {
            return false;
        }
        x -= x.signum();
        y -= 1;
    }
}

fn solve_1((x1, x2, y1, y2): (i32, i32, i32, i32)) {
    let mut max_h = 0;
    /* x velocity is 0 after enough steps.
     * If you want to go highest, take it so that you halt within the target x range
     * abs(y2) is a starting guess for y to minimize iterations
     */
    let (start_x, mut start_y) = (
        ((1_f32 + ((1 + 8 * x1) as f32).sqrt().ceil()) / 2_f32) as i32,
        y2.abs(),
    );
    // limit to abs(y1) attempts, since some steps can fall in target despite previous y vel overshot
    // empirically found that this is good enough.
    while start_y < y1.abs() {
        let start = (start_x, start_y);
        if hits_target(start, (x1, x2, y1, y2)) {
            max_h = max(max_h, start.1 * (start.1 + 1) / 2)
        }
        start_y += 1;
    }
    println!("{:?}", max_h);
}

fn solve_2((x1, x2, y1, y2): (i32, i32, i32, i32)) {
    let count = iproduct![(0..=x2), (y1..=y1.abs())]
        .filter(|(x, y)| hits_target((*x, *y), (x1, x2, y1, y2)))
        .count();
    println!("{:?}", count);
}

fn main() {
    let buf = fs::read_to_string("./input.txt").unwrap();
    let (x1, x2, y1, y2): (i32, i32, i32, i32);
    scan!(buf.bytes() => "target area: x={}..{}, y={}..{}",x1,x2,y1,y2);
    solve_1((x1, x2, y1, y2));
    solve_2((x1, x2, y1, y2));
}
