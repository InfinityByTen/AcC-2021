use std::{collections::HashMap, fs};

fn solve(input: &Vec<Vec<char>>) {
    let starts = vec!['(', '<', '[', '{'];
    let closings = HashMap::from([('(', ')'), ('{', '}'), ('[', ']'), ('<', '>')]);
    let points = HashMap::from([(')', 3), ('}', 1197), (']', 57), ('>', 25137)]);
    let autocomplete = HashMap::from([(')', 1), ('}', 3), (']', 2), ('>', 4)]);

    let mut corrupt_count = 0;
    let mut complete_count = Vec::new();
    input.iter().for_each(|seq| {
        let mut corrupted = false;
        let mut stack = Vec::new();
        for c in seq {
            if stack.is_empty() || starts.contains(c) {
                stack.push(c);
            } else {
                let last = stack.last().unwrap();
                if c == &closings[last] {
                    stack.pop();
                } else {
                    corrupt_count += points[c];
                    corrupted = true;
                    break;
                }
            }
        }
        if !stack.is_empty() && !corrupted {
            complete_count.push(
                stack
                    .iter()
                    .rev()
                    .fold(0_usize, |acc, c| (acc * 5) + autocomplete[&closings[c]]),
            );
        }
    });
    println!("{:?}", corrupt_count);
    let med = complete_count.len() / 2;
    println!("{:?}", complete_count.select_nth_unstable(med).1);
}

fn main() {
    let buf = fs::read_to_string("./input.txt").unwrap();
    let input = buf
        .split('\n')
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    solve(&input);
}
