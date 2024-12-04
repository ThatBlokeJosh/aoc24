use std::{char, usize};
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use grid::{grid, Grid};
use regex::Regex;

#[derive(Debug, Copy, Clone)]
struct Direction {
    x: i32,
    y: i32,
}


#[derive(Debug, Copy, Clone)]
struct Step {
    x: usize,
    y: usize,
    l1: char,
    l2: char,
}


pub fn part1() -> std::io::Result<()> {
    let file = File::open("./src/inputs/4.txt")?;
    let reader = BufReader::new(file);

    let mut lines: Grid<char> = grid![];

    for line in reader.lines()  {
        let content = line.unwrap();
        let letters: Vec<char> = content.chars().collect();
        lines.push_row(letters);
    }

    let directions: Vec<Direction> = vec![
        Direction{x:1,y:0},
        Direction{x:-1,y:0},
        Direction{x:0,y:1},
        Direction{x:0,y:-1},
        Direction{x:1,y:1},
        Direction{x:-1,y:-1},
        Direction{x:1,y:-1},
        Direction{x:-1,y:1},
    ];

    let mut counter = 0;

    for ((row, col), letter) in lines.indexed_iter() {
        if letter != &'X' && letter != &'S' {
            continue;
        }
        for dir in &directions {
            let mut word = "".to_string();
            for i in 0..4 {
                let x = row as i32 - (dir.x*i);
                let y = col as i32 - (dir.y*i);
                let letter = lines.get(x, y);
                match letter {
                   Some(l) => {word.push(*l)}
                   _ => {break;}
                }
            }

            if word == "XMAS" {
                counter += 1;
            }

        }
    }

    println!("{:?}", counter);

    return Ok(());
}

pub fn part2() -> std::io::Result<()> {
    let file = File::open("./src/inputs/4.txt")?;
    let reader = BufReader::new(file);

    let mut lines: Grid<char> = grid![];

    for line in reader.lines()  {
        let content = line.unwrap();
        let letters: Vec<char> = content.chars().collect();
        lines.push_row(letters);
    }

    let mut counter = 0;

    let steps = vec![
        Step{x: 0, y: 0, l1: 'M', l2: 'S'},
        Step{x: 2, y: 0, l1: 'M', l2: 'S'},
        Step{x: 1, y: 1, l1: 'A', l2: ' '},
        Step{x: 2, y: 2, l1: 'M', l2: 'S'},
        Step{x: 0, y: 2, l1: 'M', l2: 'S'},

    ];

    'a: for ((row, col), letter) in lines.indexed_iter() {
        if letter != &'M' && letter != &'S' {
            continue;
        }

        let mut chars = [' '; 5];

        for (i, step) in steps.iter().enumerate() {
            let letter = lines.get(row + step.x, col + step.y);

            match letter {
               Some(l) => {
                if *l == step.l1 || *l == step.l2 {
                } else {continue 'a;}
                chars[i] = *l;
               }
               _ => {continue 'a;}
            }
        }

        counter += if chars[0] != chars[3] && chars[1] != chars[4] {1} else {0};
    }

    println!("{:?}", counter);

    return Ok(());
}
