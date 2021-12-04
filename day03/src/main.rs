use std::fs;

fn most_common(nums: &Vec<Vec<char>>, i: usize) -> char {
    let report_len = nums.len();
    let counts = nums.iter().map(|x| x[i]).filter(|x| x == &'1').count();
    match counts > (report_len - counts) || counts == (report_len - counts) {
        true => '1',
        false => '0',
    }
}

fn vec_to_num(v: &Vec<char>) -> u32 {
    u32::from_str_radix(v.iter().collect::<String>().as_str(), 2).unwrap()
}

fn solve_1(nums: &Vec<Vec<char>>) {
    let l: usize = nums[0].len();
    let mut derived = vec!['0'; l];
    for i in 0..l {
        derived[i] = most_common(nums, i);
    }
    let gamma = vec_to_num(&derived);
    let eps = gamma ^ vec_to_num(&vec!['1'; l]);
    println!("{:?}", gamma * eps);
}

fn solve_2(nums: &Vec<Vec<char>>) {
    let mut o2 = nums.clone();
    let mut co2 = nums.clone();
    for i in 0..nums[0].len() {
        if o2.len() != 1 {
            o2 = o2
                .iter()
                .cloned()
                .filter(|x| x[i] == most_common(&o2, i))
                .collect();
        }
        if co2.len() != 1 {
            co2 = co2
                .iter()
                .cloned()
                .filter(|x| x[i] != most_common(&co2, i))
                .collect();
        }
    }
    println!("{:?}", vec_to_num(&o2[0]) * vec_to_num(&co2[0]));
}

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let nums = input
        .split('\n')
        .map(|n| n.to_string().chars().collect())
        .collect::<Vec<Vec<char>>>();
    solve_1(&nums);
    solve_2(&nums);
}
