use itertools::{iproduct, Itertools};
use std::collections::HashSet;
use std::fs;
use text_io::scan;

fn process_fold(dots: &mut Vec<(usize, usize)>, ins: (&str, usize)) {
    match ins {
        ("x", val) => dots.iter_mut().for_each(|(i, _)| {
            if *i > val {
                *i = val - (*i - val)
            }
        }),
        (_, val) => dots.iter_mut().for_each(|(_, j)| {
            if *j > val {
                *j = val - (*j - val)
            }
        }),
    }
}

fn solve_1(mut input: Vec<(usize, usize)>, instructions: &Vec<(String, usize)>) {
    let first = (instructions[0].0.as_ref(), instructions[0].1);
    process_fold(&mut input, first);
    let part1 = input.iter().unique().count();
    println!("{:?}", part1);
}

fn solve_2(mut input: Vec<(usize, usize)>, instructions: &Vec<(String, usize)>) {
    instructions
        .iter()
        .for_each(|ins| process_fold(&mut input, (ins.0.as_ref(), ins.1)));
    let code = input.iter().unique().collect::<HashSet<&(usize, usize)>>();

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

    let points = input
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

    solve_1(points.clone(), &instructions);
    solve_2(points, &instructions);
}
