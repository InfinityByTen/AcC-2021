use std::fs;
use text_io::scan;

fn solve_1(route: impl Iterator<Item = (String, isize)>) {
    let (x, y): (isize, isize) = route.fold((0, 0), |(x, y), (ins, v)| match (ins.as_str(), v) {
        ("forward", v) => (x + v, y),
        ("up", v) => (x, y - v),
        ("down", v) => (x, y + v),
        (_, _) => unreachable!(),
    });
    println!("{:?}", x * y);
}

fn solve_2(route: impl Iterator<Item = (String, isize)>) {
    let (x, y, _): (isize, isize, isize) =
        route.fold((0, 0, 0), |(x, y, a), (ins, v)| match (ins.as_str(), v) {
            ("forward", v) => (x + v, y + (v * a), a),
            ("up", v) => (x, y, a - v),
            ("down", v) => (x, y, a + v),
            (_, _) => unreachable!(),
        });
    println!("{:?}", x * y);
}

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let route = input.split('\n').map(|x| {
        let (ins, v): (String, isize);
        scan!(x.bytes() => "{} {}",ins,v);
        (ins, v)
    });
    solve_1(route.clone());
    solve_2(route);
}
