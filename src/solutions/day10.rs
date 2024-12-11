use std::{char, i8, usize};
use std::collections::{HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};
use grid::{grid, Grid};

#[derive(Debug, Copy, Clone)]
struct Location {
    x: i8,
    y: i8,
}

fn bfs(start: Location, target: u32, grid: &Grid<u32>, directions: &Vec<Location>, seen: &mut HashSet<(i8, i8)>) {
    for dir in directions {
        let loc = Location{x: start.x + dir.x, y: start.y + dir.y}; 
        match grid.get(loc.y, loc.x) {
            Some(n) => {
                if *n == 9 && target == 9 {
                    seen.insert((loc.x, loc.y));
                } else if *n == target {
                    bfs(loc, target + 1, grid, directions, seen);
                } 
            } 
            None => {continue;}
        }
    }
}


fn bfs2(start: Location, target: u32, grid: &Grid<u32>, directions: &Vec<Location>, seen: &mut i32) {
    for dir in directions {
        let loc = Location{x: start.x + dir.x, y: start.y + dir.y}; 
        match grid.get(loc.y, loc.x) {
            Some(n) => {
                if *n == 9 && target == 9 {
                    *seen += 1;
                } else if *n == target {
                    bfs2(loc, target + 1, grid, directions, seen);
                } 
            } 
            None => {continue;}
        }
    }
}

pub fn part1() -> std::io::Result<()> {
    let file = File::open("./src/inputs/10.txt")?;
    let reader = BufReader::new(file);

    let mut grid: Grid<u32> = grid![];
    let mut heads: Vec<Location> = vec![];

    for (i, line) in reader.lines().enumerate()  {
        let content = line.unwrap();
        let mut row: Vec<u32> = vec![];
        for (j, digit) in content.chars().map(|d| d.to_digit(10).unwrap()).enumerate() {
            match digit {
               0 => {
                   heads.push(Location{x: j as i8, y: i as i8});
               }
               _ => {}
            } 
            row.push(digit);
        }
        grid.push_row(row);
    }

    let directions: Vec<Location> = vec![
        Location{x:1,y:0},
        Location{x:-1,y:0},
        Location{x:0,y:1},
        Location{x:0,y:-1},
    ];

    let mut counter = 0;

    for head in heads {
        let mut seen: HashSet<(i8, i8)> = HashSet::new();
        bfs(head, 1, &grid, &directions, &mut seen);
        counter += seen.len();
    }

    println!("{:?}", counter);

    return Ok(());
}

pub fn part2() -> std::io::Result<()> {
    let file = File::open("./src/inputs/10.txt")?;
    let reader = BufReader::new(file);

    let mut grid: Grid<u32> = grid![];
    let mut heads: Vec<Location> = vec![];

    for (i, line) in reader.lines().enumerate()  {
        let content = line.unwrap();
        let mut row: Vec<u32> = vec![];
        for (j, digit) in content.chars().map(|d| d.to_digit(10).unwrap()).enumerate() {
            match digit {
               0 => {
                   heads.push(Location{x: j as i8, y: i as i8});
               }
               _ => {}
            } 
            row.push(digit);
        }
        grid.push_row(row);
    }

    let directions: Vec<Location> = vec![
        Location{x:1,y:0},
        Location{x:-1,y:0},
        Location{x:0,y:1},
        Location{x:0,y:-1},
    ];

    let mut counter = 0;

    for head in heads {
        bfs2(head, 1, &grid, &directions, &mut counter);
    }

    println!("{:?}", counter);

    return Ok(());
}
