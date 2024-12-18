use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::i32;
use std::io::{BufRead, BufReader};

pub fn part1() -> std::io::Result<()> {
    let file = File::open("./src/inputs/5.txt")?;
    let reader = BufReader::new(file);
    let mut rules = true;

    let mut rules_map: HashMap<i32, Vec<i32>> = HashMap::new();

    let mut counter = 0;

    'a: for line in reader.lines() {
        let content = line.unwrap();
        let mut seen: HashSet<i32> = HashSet::new();

        if content.is_empty() {
            rules = false;
            continue;
        }

        if rules {
            let numbers = content.split_once('|').unwrap();
            let key = numbers.0.parse::<i32>().unwrap();
            let value = numbers.1.parse::<i32>().unwrap();
            let mut r: Vec<i32> = Vec::new();
            if let Some(v) = rules_map.get_mut(&key) {
                v.push(value);
            } else {
                r.push(value);
                rules_map.insert(key, r);
            }
        }

        if !rules {
            let split: Vec<&str> = content.split(',').collect();
            let mut row: Vec<i32> = vec![];
            for n in split {
                let number = n.parse::<i32>().unwrap();
                row.push(number);
                let r = rules_map.get_mut(&number);
                if let Some(r) = r {
                    for rule in r {
                        if seen.contains(rule) {
                            continue 'a;
                        }
                    }
                }
                seen.insert(number);
            }
            counter += row[row.len() / 2];
        }
    }

    println!("{counter:?}");

    Ok(())
}

fn filter_lines(reader: BufReader<File>) -> Vec<String> {
    let mut rules = true;

    let mut rules_map: HashMap<i32, Vec<i32>> = HashMap::new();

    let mut filtered: Vec<String> = vec![];

    'a: for line in reader.lines() {
        let content = line.unwrap();
        let mut seen: HashSet<i32> = HashSet::new();

        if content.is_empty() {
            rules = false;
            filtered.push(String::new());
            continue;
        }

        if rules {
            let numbers = content.split_once('|').unwrap();
            let key = numbers.0.parse::<i32>().unwrap();
            let value = numbers.1.parse::<i32>().unwrap();
            let mut r: Vec<i32> = Vec::new();
            if let Some(v) = rules_map.get_mut(&key) {
                v.push(value);
            } else {
                r.push(value);
                rules_map.insert(key, r);
            }

            filtered.push(content);
            continue;
        }

        if !rules {
            let split: Vec<&str> = content.split(',').collect();
            for n in split {
                let number = n.parse::<i32>().unwrap();
                let r = rules_map.get_mut(&number);
                if let Some(r) = r {
                    for rule in r {
                        if seen.contains(rule) {
                            filtered.push(content);
                            continue 'a;
                        }
                    }
                }
                seen.insert(number);
            }
        }
    }

    filtered
}

pub fn part2() -> std::io::Result<()> {
    let file = File::open("./src/inputs/5.txt")?;
    let reader = BufReader::new(file);
    let mut rules = true;

    let mut rules_map: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut grid: Vec<Vec<i32>> = vec![];

    let mut counter = 0;

    'a: for content in filter_lines(reader) {
        if content.is_empty() {
            rules = false;
            continue;
        }

        if rules {
            let numbers = content.split_once('|').unwrap();
            let key = numbers.0.parse::<i32>().unwrap();
            let value = numbers.1.parse::<i32>().unwrap();
            let mut r: Vec<i32> = Vec::new();
            if let Some(v) = rules_map.get_mut(&value) {
                v.push(key);
            } else {
                r.push(key);
                rules_map.insert(value, r);
            }
        }

        if !rules {
            let split: Vec<&str> = content.split(',').collect();
            let mut row: Vec<i32> = vec![];
            for n in split {
                let number = n.parse::<i32>().unwrap();
                match rules_map.get(&number) {
                    Some(_v) => {}
                    None => {
                        rules_map.insert(number, Vec::new());
                    }
                }
                row.push(number);
            }
            grid.push(row);
        }
    }

    for updates in grid {
        let mut row = updates;
        row.sort_by_key(|a| rules_map.get(a).unwrap().len());
        counter += row[row.len() / 2];
    }

    println!("{counter:?}");

    Ok(())
}
