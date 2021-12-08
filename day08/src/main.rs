use itertools::Itertools;
use std::fs;

fn solve_1(input: &Vec<(&str, &str)>) {
    let total = input.iter().fold(0, |acc, (_, out)| {
        acc + out
            .split(' ')
            .filter(|x| x.len() == 2 || x.len() == 3 || x.len() == 4 || x.len() == 7)
            .count()
    });
    println!("{:?}", total);
}

fn solve_2(input: &Vec<(&str, &str)>) {
    let res = input.iter().fold(0, |acc, (a, b)| {
        let mut nos = [""; 10];
        let mut sixes = Vec::<&str>::new();
        let mut fives = Vec::<&str>::new();
        a.split(' ').for_each(|pos| match pos.len() {
            2 => nos[1] = pos,
            3 => nos[7] = pos,
            4 => nos[4] = pos,
            7 => nos[8] = pos,
            6 => sixes.push(pos),
            5 => fives.push(pos),
            _ => unimplemented!(),
        });
        let mut places: [char; 7] = ['.', '.', '.', '.', '.', '.', '.'];
        places[0] = nos[7]
            .chars()
            .filter(|c| !nos[1].contains(&c.to_string()))
            .collect::<Vec<char>>()[0];
        nos[6] = sixes
            .iter()
            .find(|&&opt| !nos[1].chars().all(|c| opt.contains(c)))
            .unwrap();
        nos[9] = sixes
            .iter()
            .filter(|opt| opt != &&nos[6] && nos[4].chars().all(|c| opt.contains(c)))
            .collect::<Vec<&&str>>()[0];
        nos[0] = sixes
            .iter()
            .filter(|opt| opt != &&nos[6] && opt != &&nos[9])
            .collect::<Vec<&&str>>()[0];
        places[1] = nos[1]
            .chars()
            .filter(|c| !nos[6].chars().contains(c))
            .collect::<Vec<char>>()[0];
        places[2] = nos[1]
            .chars()
            .filter(|c| c != &places[1])
            .collect::<Vec<char>>()[0];
        nos[2] = fives
            .iter()
            .filter(|opt| !opt.contains(places[2]))
            .collect::<Vec<&&str>>()[0];
        places[4] = nos[8]
            .chars()
            .filter(|c| !nos[9].contains(&c.to_string()))
            .collect::<Vec<char>>()[0];
        nos[5] = fives
            .iter()
            .filter(|opt| opt.chars().all(|c| c == places[4] || nos[6].contains(c)))
            .collect::<Vec<&&str>>()[0];
        places[6] = nos[8]
            .chars()
            .filter(|c| !nos[0].contains(&c.to_string()))
            .collect::<Vec<char>>()[0];
        nos[3] = fives
            .iter()
            .filter(|opt| opt != &&nos[5] && opt != &&nos[2])
            .collect::<Vec<&&str>>()[0];
        let output = b
            .split(' ')
            .map(|num| {
                nos.iter()
                    .enumerate()
                    .find(|x| {
                        x.1.chars().sorted().collect::<String>()
                            == num.chars().sorted().collect::<String>()
                    })
                    .unwrap()
                    .0
                    .to_string()
            })
            .collect::<String>()
            .parse::<usize>()
            .unwrap();
        acc + output
    });
    println!("{:?}", res);
}

fn main() {
    let buf = fs::read_to_string("./input.txt").unwrap();
    let input = buf
        .split('\n')
        .map(|l| l.split(" | ").collect_tuple::<(&str, &str)>().unwrap())
        .collect::<Vec<(&str, &str)>>();
    solve_1(&input);
    solve_2(&input);
}
