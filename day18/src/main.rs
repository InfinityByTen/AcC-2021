use itertools::Itertools;
use std::cmp;
use std::fs;

#[derive(Debug, Clone, PartialEq)]
enum Pair {
    Leaf(u32),
    Branch { left: Box<Pair>, right: Box<Pair> },
}

impl Pair {
    fn magnitude(&self) -> usize {
        match self {
            Pair::Leaf(val) => *val as usize,
            Pair::Branch { left: l, right: r } => 3 * l.magnitude() + 2 * r.magnitude(),
        }
    }

    fn absorb(&mut self, from_left: bool, val: u32) {
        match self {
            Pair::Leaf(prev) => *prev += val,
            Pair::Branch { left: l, right: r } => match from_left {
                true => l.absorb(from_left, val),
                false => r.absorb(from_left, val),
            },
        }
    }

    fn reduce(&mut self, depth: u32) -> Option<(u32, u32)> {
        if let Pair::Leaf(_) = self {
            // println!("Do nothing for {:?}", val);
            None
        } else if let Pair::Branch { left: l, right: r } = self {
            if depth == 4 {
                let a = match **l {
                    Pair::Leaf(val) => val,
                    _ => unreachable!(),
                };
                let b = match **r {
                    Pair::Leaf(val) => val,
                    _ => unreachable!(),
                };
                *self = Pair::Leaf(0);
                return Some((a, b));
            } else {
                if let Some((a, b)) = l.reduce(depth + 1) {
                    // println!("Left explodes giving {:?} to absorb in {:?}", (a, b), r);
                    r.absorb(true, b);
                    return Some((a, 0));
                }
                if let Some((a, b)) = r.reduce(depth + 1) {
                    // println!("Right explodes giving {:?} to absorb in {:?}", (a, b), l);
                    l.absorb(false, a);
                    return Some((0, b));
                } else {
                    None
                }
            }
        } else {
            None
        }
    }

    fn split(&mut self) -> Option<()> {
        match self {
            Pair::Leaf(val) => {
                if *val >= 10 {
                    *self = Pair::Branch {
                        left: Box::new(Pair::Leaf((*val as f32 / 2.0).floor() as u32)),
                        right: Box::new(Pair::Leaf((*val as f32 / 2.0).ceil() as u32)),
                    };
                    return Some(());
                } else {
                    return None;
                }
            }
            Pair::Branch { left, right } => {
                if let Some(_) = left.split() {
                    return Some(());
                }
                if let Some(_) = right.split() {
                    return Some(());
                }
                return None;
            }
        }
    }
}

fn get_pair(input: &str) -> Pair {
    if let Ok(num) = input.parse::<u32>() {
        Pair::Leaf(num)
    } else {
        let mut depth = 0;
        let mut split_point = 0;
        for (pos, c) in input.chars().enumerate() {
            match c {
                '[' => depth += 1,
                ']' => depth -= 1,
                ',' if depth == 1 => split_point = pos,
                _ => (),
            }
        }
        Pair::Branch {
            left: Box::new(get_pair(&input[1..split_point]).to_owned()),
            right: Box::new(get_pair(&input[(split_point + 1)..(input.len() - 1)]).to_owned()),
        }
    }
}

fn reduced_addition(x: &Pair, y: &Pair) -> Pair {
    let mut res = Pair::Branch {
        left: Box::new(x.clone()),
        right: Box::new(y.clone()),
    };
    while res.reduce(0).is_some() || res.split().is_some() {
        // println!("Continuing with {:?}", res);
    }
    res
}

fn solve_1(input: &Vec<Pair>) {
    let res = input
        .iter()
        .skip(1)
        .fold(input[0].clone(), |acc, nxt| reduced_addition(&acc, &nxt));
    println!("{:?}", res.magnitude());
}

fn solve_2(input: &Vec<Pair>) {
    let limit = input.len() - 1;
    let res = (0..limit).permutations(2).fold(0, |acc, e| {
        let a_p_b = reduced_addition(&input[e[0]], &input[e[1]]).magnitude();
        let b_p_a = reduced_addition(&input[e[0]], &input[e[1]]).magnitude();
        cmp::max(acc, a_p_b.max(b_p_a))
    });
    println!("{:?}", res);
}

fn main() {
    let buf = fs::read_to_string("./input.txt").unwrap();
    let input = buf.split('\n').map(|x| get_pair(x)).collect::<Vec<Pair>>();
    solve_1(&input);
    solve_2(&input);
}
