use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::{i32, usize};
use std::io::{BufRead, BufReader};

use grid::{grid, Grid};

pub fn part1() -> std::io::Result<()> {
    let file = File::open("./src/inputs/5.txt")?;
    let reader = BufReader::new(file);
    let mut rules = true;

    let mut rules_map: HashMap<i32, Vec<i32>> = HashMap::new();


    let mut counter = 0;

    'a: for line in reader.lines()  {
        let content = line.unwrap();
        let mut seen: HashSet<i32> = HashSet::new();

        if content == "" {
            rules = false;
            continue;
        }

        if rules {
            let numbers = content.split_once("|").unwrap();
            let key = numbers.0.parse::<i32>().unwrap();
            let value = numbers.1.parse::<i32>().unwrap();
            let mut r: Vec<i32> = Vec::new();
            match rules_map.get_mut(&key) {
               Some(v) => {v.push(value);}
               None => {r.push(value); rules_map.insert(key, r);} 
            }
        }

        if !rules {
            let split: Vec<&str> = content.split(",").collect();
            let mut row: Vec<i32> = vec![];
            for n in split {
               let number = n.parse::<i32>().unwrap();
               row.push(number);
               let r = rules_map.get_mut(&number);
               match r {
                   Some(r) => {
                       for rule in r {
                            if seen.contains(rule) {
                                continue 'a;
                            }
                       }
                    }
                   None => {}
               }
               seen.insert(number);
            }
            counter += row[row.len()/2];
        }
    }



    println!("{:?}", counter);

    return Ok(());
}

fn filter_lines(reader: BufReader<File>) -> Vec<String> {
    let mut rules = true;

    let mut rules_map: HashMap<i32, Vec<i32>> = HashMap::new();

    let mut filtered: Vec<String> = vec![];

    'a: for line in reader.lines()  {
        let content = line.unwrap();
        let mut seen: HashSet<i32> = HashSet::new();

        if content == "" {
            rules = false;
            filtered.push("".to_string());
            continue;
        }

        if rules {
            let numbers = content.split_once("|").unwrap();
            let key = numbers.0.parse::<i32>().unwrap();
            let value = numbers.1.parse::<i32>().unwrap();
            let mut r: Vec<i32> = Vec::new();
            match rules_map.get_mut(&key) {
               Some(v) => {v.push(value);}
               None => {r.push(value); rules_map.insert(key, r);} 
            }

            filtered.push(content);
            continue;
        }

        if !rules {
            let split: Vec<&str> = content.split(",").collect();
            for n in split {
               let number = n.parse::<i32>().unwrap();
               let r = rules_map.get_mut(&number);
               match r {
                   Some(r) => {
                       for rule in r {
                            if seen.contains(rule) {
                                filtered.push(content); 
                                continue 'a;
                            }
                       }
                    }
                   None => {}
               }
               seen.insert(number);
            }
        }
    }

    return filtered;
}


pub fn part2() -> std::io::Result<()> {
    let file = File::open("./src/inputs/5.txt")?;
    let reader = BufReader::new(file);
    let mut rules = true;

    let mut rules_map: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut grid: Vec<Vec<i32>> = vec![]; 

    let mut counter = 0;

    'a: for content in filter_lines(reader)  {
        if content == "" {
            rules = false;
            continue;
        }

        if rules {
            let numbers = content.split_once("|").unwrap();
            let key = numbers.0.parse::<i32>().unwrap();
            let value = numbers.1.parse::<i32>().unwrap();
            let mut r: Vec<i32> = Vec::new();
            match rules_map.get_mut(&value) {
               Some(v) => {v.push(key);}
               None => {r.push(key); rules_map.insert(value, r);} 
            }
        }

        if !rules {
            let split: Vec<&str> = content.split(",").collect();
            let mut row: Vec<i32> = vec![];
            for n in split {
               let number = n.parse::<i32>().unwrap();
               row.push(number);
            }
            grid.push(row);
        }
    }

    let mut m = 0;

    println!("{:?}", rules_map);

    for mut updates in grid {
        let mut row: Vec<i32> = vec![];
        let mut rules = rules_map.clone();
        while row.len() != updates.len() {
            for (j, page) in updates.iter().enumerate() {
                match rules.get_mut(&page) {
                    Some(r) => {
                        for i in 0..r.len() {
                           if row.contains(&r[i]) || !updates.contains(&r[i]) {
                                r.remove(i);
                                break;
                           } 
                        }
                        if r.len() == 0 {
                            if !row.contains(page) {
                                row.push(*page);
                            }
                        }
                    }
                    None => {
                        if !row.contains(page) {
                            row.push(*page);
                        }
                    } 
                }
            }
        }
        counter += row[row.len()/2];
        m = std::cmp::max(row.len(), m);
    }


    println!("{:?}", counter);
    println!("{:?}", m);

    return Ok(());
}

