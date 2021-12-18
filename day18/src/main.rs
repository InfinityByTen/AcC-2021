use itertools::iproduct;
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
            Pair::Branch { left: l, right: r } => {
                if from_left {
                    l.absorb(from_left, val);
                } else {
                    r.absorb(from_left, val)
                }
            }
        }
    }

    fn reduce(&mut self, depth: u32) -> Option<(u32, u32)> {
        // println!("{:?}", (&self, depth));
        if let Pair::Leaf(_val) = self {
            // println!("Do nothing for {:?}", val);
            None
        } else if let Pair::Branch { left: l, right: r } = self {
            if depth == 4 {
                // println!("Exploding {:?}", self.clone());
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
                    // println!("Right explodes giving {:?} to absorb in {:?}", (a, b), r);
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

#[allow(dead_code)]
fn add_pair(a: &Pair, b: &Pair) -> Pair {
    Pair::Branch {
        left: Box::new(a.clone()),
        right: Box::new(b.clone()),
    }
}

fn reduced_addition(x: &Pair, y: &Pair) -> Pair {
    let mut res = add_pair(&x, &y);
    while res.reduce(0).is_some() || res.split().is_some() {
        // println!("Continuing with {:?}", res);
    }
    res
}

fn solve_1(input: &Vec<&str>) {
    let res = input.iter().skip(1).fold(get_pair(input[0]), |acc, nxt| {
        reduced_addition(&acc, &get_pair(&nxt))
    });
    println!("{:?}", res.magnitude());
}

fn solve_2(input: &Vec<&str>) {
    let limit = input.len() - 1;
    let res = iproduct![0..limit, 0..limit]
        .filter(|(a, b)| a != b)
        .fold(0, |acc, (a, b)| {
            let a_p_b = reduced_addition(&get_pair(input[a]), &get_pair(input[b])).magnitude();
            let b_p_a = reduced_addition(&get_pair(input[b]), &get_pair(input[a])).magnitude();
            cmp::max(acc, a_p_b.max(b_p_a))
        });
    println!("{:?}", res);
}

fn main() {
    let buf = fs::read_to_string("./input.txt").unwrap();
    let input = buf.split('\n').collect::<Vec<&str>>();
    solve_1(&input);
    solve_2(&input);
}
