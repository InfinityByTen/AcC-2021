use itertools::Itertools;
use std::collections::HashMap;
use std::fs;

fn solve_1(input: &(&str, &str)) {
    let rules = input
        .1
        .split('\n')
        .map(|r| {
            let rule = r.split(" -> ").collect_tuple::<(&str, &str)>().unwrap();
            (rule.0.to_string(), rule.1.chars().nth(0).unwrap())
        })
        .collect::<HashMap<String, char>>();
    println!("{:?}", rules);

    let mut bam = input.0.to_string();

    (0..10).for_each(|_| {
        let template = bam.chars().collect::<Vec<char>>();
        // println!("{:?}", bam);

        let elements = template
            .windows(2)
            .map(|a| a.iter().collect::<String>())
            .collect::<Vec<String>>();

        let mut dummy = Vec::new();
        (0..template.len() - 1).for_each(|pos| {
            dummy.push(template[pos]);
            dummy.push(*rules.get(&elements[pos]).unwrap());
        });
        dummy.push(template[template.len() - 1]);
        bam = dummy.iter().collect();
    });

    let mut freq = HashMap::new();
    bam.chars().for_each(|c| {
        let _ = freq.entry(c).and_modify(|f| *f += 1).or_insert(1);
    });
    println!("{:?}", freq);
    println!(
        "{:?}",
        freq.values().max().unwrap() - freq.values().min().unwrap()
    );
}

fn main() {
    let buf = fs::read_to_string("./test.txt").unwrap();
    let input = buf.split("\n\n").collect_tuple::<(&str, &str)>().unwrap();
    solve_1(&input);
}
