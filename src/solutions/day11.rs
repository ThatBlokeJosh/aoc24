use std::collections::{HashMap};
use std::fs::{self};
use std::{u64, vec};

pub fn part1() -> std::io::Result<()> {
    let line = fs::read_to_string("./src/inputs/11.txt").unwrap();
    let mut numbers: Vec<u64> = line.trim().split(" ").map(|n| n.parse::<u64>().unwrap()).collect();

    for _i in 0..25 {
        let mut new_nums: Vec<u64> = vec![]; 
        for n in numbers {
            match n {
               0 => {new_nums.push(1);} 
               _ => {
                    let digits = n.ilog10() + 1;
                    if digits % 2 == 0 {
                        let half = (10 as u64).pow(digits / 2);
                        new_nums.push(n / half);
                        new_nums.push(n % half);
                    } else {
                        new_nums.push(n*2024);
                    }
               }
            }
        }
        numbers = new_nums;
    }

    println!("{:?}", numbers.len());

    return Ok(());
}

fn blink(number: u64, depth: u64, cache: &mut HashMap<(u64, u64), u64>) -> u64 {
    if depth == 75 {
        return 1;
    }

    if let Some(n) = cache.get(&(number, depth)) {
        return *n;
    }

    match number {
       0 => {
            let res = blink(1, depth + 1, cache);
            cache.insert((number, depth), res);
            return res;
       } 
       _ => {
            let digits = number.ilog10() + 1;
            if digits % 2 == 0 {
                let half = (10 as u64).pow(digits / 2);
                let res = blink(number / half, depth + 1, cache) + blink(number % half, depth + 1, cache);
                cache.insert((number, depth), res);
                return res;
            } else {
                let res = blink(number*2024, depth + 1, cache);
                cache.insert((number, depth), res);
                return res;
            }
       }
    }
}

pub fn part2() -> std::io::Result<()> {
    let line = fs::read_to_string("./src/inputs/11.txt").unwrap();
    let numbers: Vec<u64> = line.trim().split(" ").map(|n| n.parse::<u64>().unwrap()).collect();

    let mut counter = 0;
    let mut cache: HashMap<(u64, u64), u64> = HashMap::new();

    for n in numbers {
        counter += blink(n, 0, &mut cache);
    }
    println!("{:?}", counter);

    return Ok(());
}
