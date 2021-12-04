use std::fs;

#[derive(Debug, Clone)]
struct Board<'a> {
    rows: Vec<&'a str>,
    flags: Vec<bool>,
}

impl Board<'_> {
    fn mark(&mut self, num: &str) {
        for (data, marks) in self.rows.iter().zip(self.flags.iter_mut()) {
            if data == &num {
                *marks = true;
            }
        }
    }

    fn winner(&self) -> bool {
        self.flags.chunks(5).any(|row| row.iter().all(|x| *x))
            || (0..5).any(|col| self.flags.iter().skip(col).step_by(5).all(|x| *x))
    }

    fn dinner(&self, num: &str) -> u64 {
        let zipped = self.rows.iter().zip(self.flags.iter());
        zipped.fold(0_u64, |acc, (val, mark)| match mark {
            true => acc + 0,
            false => acc + val.parse::<u64>().unwrap(),
        }) * num.parse::<u64>().unwrap()
    }
}

fn solve_1(nos: Vec<&str>, mut boards: Vec<Board>) {
    for i in 0..nos.len() {
        boards.iter_mut().for_each(|b| b.mark(nos[i]));
        if let Some(board) = boards.iter().find(|b| b.winner()) {
            println!("Winner Winner {:?}", board.dinner(nos[i]));
            break;
        }
    }
}

fn solve_2(nos: Vec<&str>, mut boards: Vec<Board>) {
    for i in 0..nos.len() {
        boards.iter_mut().for_each(|b| b.mark(nos[i]));
        if boards.len() == 1 && boards[0].winner() {
            println!("Loser Loser {:?}", boards[0].dinner(nos[i]));
            break;
        } else {
            boards = boards.iter().cloned().filter(|b| !b.winner()).collect();
        }
    }
}

fn main() {
    let buf = fs::read_to_string("./input.txt").unwrap();
    let input: Vec<&str> = buf.split("\n\n").collect();
    let nos = input[0].split(',').collect::<Vec<&str>>();
    let boards = input[1..]
        .iter()
        .map(|r| Board {
            rows: r
                .split('\n')
                .map(|c| c.split(' ').filter(|e| e != &"").collect::<Vec<&str>>())
                .flatten()
                .collect::<Vec<&str>>(),
            flags: vec![false; 25],
        })
        .collect::<Vec<Board>>();
    solve_1(nos.clone(), boards.clone());
    solve_2(nos.clone(), boards.clone());
}
