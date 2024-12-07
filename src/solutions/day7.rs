use std::fs::File;
use std::io::{BufRead, BufReader};

fn concat(a: u64, b: u64) -> u64 {
    a * 10u64.pow(b.ilog10() + 1) + b
}

pub fn dfs(numbers: &Vec<u64>, index: usize, target: u64, total: u64) -> bool {
    if index >= numbers.len() {
        return target == total;
    }

    let add = dfs(numbers, index + 1, target, total + numbers[index]);
    if add {return true;}
    let mult = dfs(numbers, index + 1, target, total * numbers[index]); 
    return mult;
}

pub fn dfs_w_con(numbers: &Vec<u64>, index: usize, target: u64, total: u64) -> bool {
    if index >= numbers.len() {
        return target == total;
    }

    let add = dfs_w_con(numbers, index + 1, target, total + numbers[index]);
    if add {return true;}

    let mult = dfs_w_con(numbers, index + 1, target, total * numbers[index]); 
    if mult {return true;}

    let con = dfs_w_con(numbers, index + 1, target, concat(total, numbers[index]));
    return con;
}

pub fn part1() -> std::io::Result<()> {
    let file = File::open("./src/inputs/7.txt")?;
    let reader = BufReader::new(file);

    let mut counter: u64 = 0;

    for line in reader.lines() {
        let content = line.unwrap();
        let split = content.split_once(": ").unwrap();
        let target = split.0.parse::<u64>().unwrap();
        let numbers: Vec<u64> = split.1.split(" ").map(|x| x.parse::<u64>().unwrap()).collect();
        counter += if dfs(&numbers, 1, target, numbers[0]) {target} else {0};
    }


    println!("{:?}", counter);

    return Ok(());
}

pub fn part2() -> std::io::Result<()> {
    let file = File::open("./src/inputs/7.txt")?;
    let reader = BufReader::new(file);

    let mut counter: u64 = 0;

    for line in reader.lines() {
        let content = line.unwrap();
        let split = content.split_once(": ").unwrap();
        let target = split.0.parse::<u64>().unwrap();
        let numbers: Vec<u64> = split.1.split(" ").map(|x| x.parse::<u64>().unwrap()).collect();
        counter += if dfs_w_con(&numbers, 1, target, numbers[0]) {target} else {0};
    }


    println!("{:?}", counter);

    return Ok(());
}
