// use itertools::Itertools;
use std::fs;

#[derive(Debug, Clone)]
struct Board<'a> {
    rows: Vec<Vec<&'a str>>,
    cols: Vec<Vec<&'a str>>,
}

impl Board<'_> {
    fn mark(&mut self, num: &str) {
        self.rows = self
            .rows
            .iter()
            .map(|r| r.iter().filter(|e| e != &&num).map(|x| x.clone()).collect())
            .collect();
        self.cols = self
            .cols
            .iter()
            .map(|r| r.iter().filter(|e| e != &&num).map(|x| x.clone()).collect())
            .collect();
    }

    fn winner_winner(&self) -> bool {
        self.rows.iter().any(|r| r.len() == 0) || self.cols.iter().any(|c| c.len() == 0)
    }

    fn chicken_dinner(&self) -> u64 {
        self.rows
            .iter()
            .flatten()
            .fold(0_u64, |acc, e| acc + e.parse::<u64>().unwrap())
    }
}

fn solve_1(nos: Vec<&str>, mut boards: Vec<Board>) {
    for i in 0..nos.len() {
        boards.iter_mut().for_each(|b| b.mark(nos[i]));
        if let Some(board) = boards.iter().find(|b| b.winner_winner()) {
            println!(
                "Winner Winner: {:?}",
                board.chicken_dinner() * nos[i].parse::<u64>().unwrap()
            );
            break;
        }
    }
}

fn solve_2(nos: Vec<&str>, mut boards: Vec<Board>) {
    for i in 0..nos.len() {
        boards.iter_mut().for_each(|b| b.mark(nos[i]));
        boards = boards
            .iter()
            .filter(|b| !b.winner_winner())
            .map(|x| x.clone())
            .collect();
        if boards.len() == 1 {
            boards[0].mark(nos[i + 1]);
            println!(
                "Loser Loser {:?}",
                boards[0].chicken_dinner() * nos[i + 1].parse::<u64>().unwrap()
            );
        }
    }
}

fn main() {
    let buf = fs::read_to_string("./input.txt").unwrap();
    let input: Vec<&str> = buf.split("\n\n").collect();
    let nos = input[0].split(',').collect::<Vec<&str>>();
    let boards = input[1..]
        .iter()
        .map(|r| {
            let r_major = r
                .split('\n')
                .map(|c| c.split(' ').filter(|e| e != &"").collect::<Vec<&str>>())
                .collect::<Vec<Vec<&str>>>();
            Board {
                rows: r_major.clone(),
                cols: {
                    let mut cols = vec![vec![""; r_major[0].len()]; r_major.len()];
                    for i in 0..r_major.len() {
                        for j in 0..r_major[0].len() {
                            cols[i][j] = r_major[j][i];
                        }
                    }
                    cols
                },
            }
        })
        .collect::<Vec<Board>>();
    solve_1(nos.clone(), boards.clone());
    solve_2(nos.clone(), boards.clone());
}
