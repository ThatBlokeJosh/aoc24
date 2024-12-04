use std::fs::File;
use std::io::{BufRead, BufReader};
use std::cmp;
use std::collections::HashMap;

pub fn part1() -> std::io::Result<()> {
    let file = File::open("./src/inputs/1.txt")?;
    let reader = BufReader::new(file);

    let mut distance: i32 = 0;

    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();

    for line in reader.lines()  {
        let unwrapped = line.unwrap();
        let split: Vec<&str> = unwrapped.split("   ").collect();
        let x = split[0].parse::<i32>().unwrap();
        let y = split[1].parse::<i32>().unwrap();
        left.push(x);
        right.push(y);
    }

    left.sort();
    right.sort();

    for i in 0..left.len()  {
        distance += cmp::max(left[i], right[i]) - cmp::min(left[i], right[i]);
    }

    println!("{:?}", distance);

    return Ok(());
}

pub fn part2() -> std::io::Result<()> {
    let file = File::open("./src/inputs/1.txt")?;
    let reader = BufReader::new(file);

    let mut distance: i32 = 0;

    let mut left: Vec<i32> = Vec::new();
    let mut right: HashMap<i32, i32> = HashMap::new();

    for line in reader.lines()  {
        let unwrapped = line.unwrap();
        let split: Vec<&str> = unwrapped.split("   ").collect();
        let x = split[0].parse::<i32>().unwrap();
        let y = split[1].parse::<i32>().unwrap();
        left.push(x);
        *right.entry(y).or_insert(0) += 1;
    }

    for i in 0..left.len()  {
        let x = left[i];
        distance += x * if right.contains_key(&x) { right[&x] } else { 0 };
    }

    println!("{:?}", distance);

    return Ok(());
}
