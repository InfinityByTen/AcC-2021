use core::hash::Hash;
use itertools::Itertools;
use std::collections::HashMap;
use std::fs;

fn freq_mapper<T: Eq + Hash>(freq: &mut HashMap<T, usize>, key: T, val: usize) {
    freq.entry(key).and_modify(|c| *c += val).or_insert(val);
}

fn solve(input: &str, rules: &HashMap<String, char>, max_steps: u32) {
    let template = input.to_string();
    let mut pattern_counts = HashMap::new();
    let temp_chars = template.chars().collect::<Vec<char>>();
    temp_chars.windows(2).for_each(|a| {
        freq_mapper(&mut pattern_counts, a.iter().collect::<String>(), 1);
    });

    (0..max_steps).for_each(|_| {
        let mut dummy = HashMap::new();
        pattern_counts.iter().for_each(|pattern| {
            let evolution = rules[pattern.0.as_str()];
            let a = [pattern.0.chars().nth(0).unwrap(), evolution]
                .iter()
                .collect::<String>();
            let b = [evolution, pattern.0.chars().nth(1).unwrap()]
                .iter()
                .collect::<String>();
            freq_mapper(&mut dummy, a, *pattern.1);
            freq_mapper(&mut dummy, b, *pattern.1);
        });
        pattern_counts = dummy;
    });

    let mut char_counts = HashMap::new();
    pattern_counts.iter().for_each(|pc| {
        pc.0.chars()
            .for_each(|c| freq_mapper(&mut char_counts, c, *pc.1))
    });
    char_counts
        .entry(template.chars().nth(0).unwrap())
        .and_modify(|f| *f += 1);
    char_counts
        .entry(template.chars().nth(template.len() - 1).unwrap())
        .and_modify(|f| *f += 1);
    println!(
        "{:?}",
        (char_counts.values().max().unwrap() - char_counts.values().min().unwrap()) / 2
    );
}

fn main() {
    let buf = fs::read_to_string("./input.txt").unwrap();
    let input = buf.split("\n\n").collect_tuple::<(&str, &str)>().unwrap();
    let rules = input
        .1
        .split('\n')
        .map(|r| {
            let rule = r.split(" -> ").collect_tuple::<(&str, &str)>().unwrap();
            (rule.0.to_string(), rule.1.chars().nth(0).unwrap())
        })
        .collect::<HashMap<String, char>>();
    solve(input.0, &rules, 10);
    solve(input.0, &rules, 40);
}
