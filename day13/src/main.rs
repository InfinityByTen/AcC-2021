use itertools::{iproduct, Itertools};
use std::collections::HashSet;
use std::fs;
use text_io::scan;

fn solve_1(input: &(&str, &str)) {
    let mut points = input
        .0
        .split('\n')
        .map(|coords| {
            let (i, j): (usize, usize);
            scan!(coords.bytes()=>"{},{}",i,j);
            (i, j)
        })
        .collect::<Vec<(usize, usize)>>();

    let instructions = input
        .1
        .split('\n')
        .map(|fold| {
            let (axis, val): (String, usize);
            scan!(fold.bytes()=>"fold along {}={}", axis,val);
            (axis, val)
        })
        .collect::<Vec<(String, usize)>>();

    let mut process_fold = |ins: (&str, usize)| match ins {
        ("x", val) => points.iter_mut().for_each(|(i, _)| {
            if *i > val {
                *i = val - (*i - val)
            }
        }),
        ("y", val) => points.iter_mut().for_each(|(_, j)| {
            if *j > val {
                *j = val - (*j - val)
            }
        }),
        (_, _) => unreachable!(),
    };

    // let first = (instructions[0].0.as_ref(), instructions[0].1);
    // process_fold(first);
    // println!("{:?}", points.iter().unique().count());

    instructions
        .iter()
        .for_each(|ins| process_fold((ins.0.as_ref(), ins.1)));
    // println!("{:?}", points);

    let code = points.iter().unique().collect::<HashSet<&(usize, usize)>>();

    let (a, b) = (
        code.iter().max_by_key(|(_, j)| j).unwrap().1,
        code.iter().max_by_key(|(i, _)| i).unwrap().0,
    );

    iproduct![(0..=a), (0..=b)].for_each(|pt| {
        if code.contains(&(pt.1, pt.0)) {
            print!("|#|")
        } else {
            print!(" . ")
        }
        if pt.1 == b {
            print!("\n")
        }
    });
}

fn main() {
    let buf = fs::read_to_string("./input.txt").unwrap();
    let input = buf.split("\n\n").collect_tuple::<(&str, &str)>().unwrap();
    solve_1(&input);
}
