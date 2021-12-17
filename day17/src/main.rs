use itertools::iproduct;
use std::fs;
use text_io::scan;

fn run(
    start: (i32, i32),
    (x1, x2, y1, y2): (i32, i32, i32, i32),
    get_positions: bool,
) -> (Vec<(i32, i32)>, bool) {
    let (mut pos, mut records, mut overshoot) = ((0, 0), Vec::new(), false);
    let (mut x, mut y) = start;
    loop {
        pos = (pos.0 + x, pos.1 + y);
        if pos.0 >= x1 && pos.0 <= x2 && pos.1 >= y1 && pos.1 <= y2 {
            if get_positions {
                records.push(pos);
            }
            break;
        }
        if pos.1 < y1 {
            overshoot = true;
            break;
        }
        if get_positions {
            records.push(pos);
        }
        if x > 0 {
            x -= 1;
        } else if x < 0 {
            x += 1;
        }
        y -= 1;
    }
    (records, overshoot)
}

fn solve_1((x1, x2, y1, y2): (i32, i32, i32, i32)) {
    let mut max = ((0, 0), 0);
    /* x velocity is 0 after enough steps.
     * If you want to go highest, take it so that you halt within the target x range
     * abs(y2) is a starting guess for y to minimize iterations
     */
    let (start_x, mut start_y) = (
        (((1 + 8 * x1) as f32).sqrt().ceil() / 2_f32) as i32,
        y2.abs(),
    );
    // limit to abs(y1) attempts, since some steps can fall in target despite previous y vel overshot
    // empirically found that this is good enough.
    while start_y < y1.abs() {
        let start = (start_x, start_y);
        let res = run(start, (x1, x2, y1, y2), true);
        let max_h = res.0.iter().max_by_key(|(_, h)| h).unwrap();
        if max_h.1 > max.1 && !res.1 {
            max.0 = start;
            max.1 = max_h.1;
        }
        start_y += 1;
    }
    println!("{:?}", max.1);
}

fn solve_2((x1, x2, y1, y2): (i32, i32, i32, i32)) {
    let count = iproduct![(0..=x2), (y1..=y1.abs())]
        .filter(|(x, y)| !run((*x, *y), (x1, x2, y1, y2), false).1)
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
