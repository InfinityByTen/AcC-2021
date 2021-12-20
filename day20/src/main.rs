use itertools::{iproduct, Itertools};
use std::collections::HashSet;
use std::fs;

fn refine(
    algo: &Vec<char>,
    pix_map: &HashSet<(isize, isize)>,
    padding_on: bool,
) -> (HashSet<(isize, isize)>, bool) {
    let a = pix_map.iter().min_by_key(|(i, _)| i).unwrap().0 - 1;
    let b = pix_map.iter().max_by_key(|(i, _)| i).unwrap().0 + 1;
    let p = pix_map.iter().min_by_key(|(_, j)| j).unwrap().1 - 1;
    let q = pix_map.iter().max_by_key(|(_, j)| j).unwrap().1 + 1;

    let mut output = HashSet::new();
    (a..=b).for_each(|i| {
        (p..=q).for_each(|j| {
            let mut num = 0_usize;
            iproduct![((i - 1)..=(i + 1)), ((j - 1)..=(j + 1))].for_each(|e| {
                num = num << 1;
                num |= (((e.0 <= a || e.0 >= b || e.1 <= p || e.1 >= q) && padding_on)
                    || pix_map.contains(&e)) as usize;
            });
            if algo[num] == '#' {
                output.insert((i, j));
            }
        })
    });
    match padding_on {
        true => (output, algo[511] == '#'),
        false => (output, algo[0] == '#'),
    }
}

fn solve(algo: &Vec<char>, pix_map: &mut HashSet<(isize, isize)>, count: usize) {
    let mut start_flip = false;
    (0..count).for_each(|_| {
        let res = refine(algo, &pix_map, start_flip);
        *pix_map = res.0;
        start_flip = res.1;
    });
    println!("{:?}", pix_map.len());
}

fn main() {
    let buf = fs::read_to_string("./input.txt").unwrap();
    let input = buf.split("\n\n").collect::<Vec<&str>>();
    let algo = input[0].chars().collect::<Vec<char>>();
    let image = input[1]
        .split('\n')
        .map(|l| l.chars().collect_vec())
        .collect::<Vec<Vec<char>>>();

    let mut pix_map = HashSet::new();
    let m = image.len();
    let n = image[0].len();
    (0..m).for_each(|row| {
        (0..n).for_each(|col| {
            if image[row][col] == '#' {
                pix_map.insert((row as isize, col as isize));
            }
        })
    });

    solve(&algo, &mut pix_map.clone(), 2);
    solve(&algo, &mut pix_map, 50);
}
