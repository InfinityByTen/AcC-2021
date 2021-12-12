use itertools::Itertools;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;

#[derive(Debug, PartialEq)]
struct Edge {
    from: String,
    to: String,
}

fn get_edges(input: &Vec<(&str, &str)>, vertex: &str) -> Vec<Edge> {
    input
        .iter()
        .map(|(f, t)| {
            if *f == vertex {
                Some(Edge {
                    from: vertex.to_string(),
                    to: t.to_string(),
                })
            } else if *t == vertex {
                Some(Edge {
                    from: vertex.to_string(),
                    to: f.to_string(),
                })
            } else {
                None
            }
        })
        .filter(|x| x.is_some())
        .map(|e| e.unwrap())
        .collect::<Vec<Edge>>()
}

fn solve(input: &Vec<(&str, &str)>, is_relaxed: bool) {
    let mut paths = HashSet::from([vec!["start".to_string()]]);
    let mut queue = VecDeque::from(["start".to_string()]);
    let mut completed = 0;

    let small_frequencies = |path: &Vec<String>| {
        let mut freq = HashMap::new();
        path.iter().for_each(|v| {
            if v.chars().any(|c| c.is_ascii_lowercase()) {
                freq.entry(v.clone()).and_modify(|c| *c += 1).or_insert(1);
            }
        });
        freq
    };

    let part = |path: &Vec<String>, v: &String, is_relaxed: bool| match is_relaxed {
        true => {
            v != "start"
                && (v.chars().any(|c| c.is_ascii_uppercase())
                    || !path.contains(&v)
                    || !small_frequencies(path).values().any(|c| c > &1))
        }
        false => !(path.contains(&v) && v.chars().any(|c| c.is_ascii_lowercase())),
    };

    let mut get_count = 0;
    while !queue.is_empty() {
        let vertex = queue.pop_front().unwrap();
        if vertex == "end" {
            continue;
        }

        let mut to_purge = Vec::new();
        get_edges(input, &vertex).iter().for_each(|e| {
            let mut temp = Vec::new();
            paths.iter().for_each(|path| {
                get_count += 1;
                let path_end = path.last().unwrap();
                if path_end == &e.from && part(path, &e.to, is_relaxed) {
                    let mut dummy = path.clone();
                    to_purge.push(path.clone());
                    dummy.push(e.to.clone());
                    temp.push(dummy);
                    if !queue.contains(&e.to) {
                        queue.push_back(e.to.clone());
                    }
                }
            });

            temp.iter().cloned().for_each(|t| {
                if e.to == "end" {
                    completed += 1;
                } else {
                    let _ = paths.insert(t);
                }
            });
        });
        to_purge.iter().for_each(|p| {
            let _ = paths.remove(p);
        });
    }
    println!("{:?} with {:?} paths analysed", completed, get_count);
}

fn main() {
    let buf = fs::read_to_string("./input.txt").unwrap();
    let input = buf
        .split('\n')
        .map(|l| l.split('-').collect_tuple().unwrap())
        .collect::<Vec<(&str, &str)>>();
    solve(&input, false);
    solve(&input, true);
}
