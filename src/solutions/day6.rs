use std::collections::HashSet;
use std::{char};
use std::fs::File;
use std::io::{BufRead, BufReader};
use grid::{grid, Grid};

#[derive(Debug, Copy, Clone)]
enum Block {
    Obstacle,
    Empty, 
}

#[derive(Debug, Copy, Clone)]
struct Direction {
    x: i32,
    y: i32,
}


pub fn part1() -> std::io::Result<()> {
    let file = File::open("./src/inputs/6.txt")?;
    let reader = BufReader::new(file);

    let mut grid: Grid<Block> = grid![];
    let mut guard_location = Direction{x: 0, y: 0};

    for (i, line) in reader.lines().enumerate()  {
        let content = line.unwrap();
        let mut row: Vec<Block> = vec![];
        for (j, ch) in content.chars().enumerate() {
           match ch {
              '.' => {row.push(Block::Empty);}
              '#' => {row.push(Block::Obstacle);}
              '^' => {row.push(Block::Empty); guard_location = Direction{x: j as i32, y: i as i32};}
              _ => {} 
           } 
        }
        grid.push_row(row);
    }

    let mut counter: HashSet<(i32, i32)> = HashSet::new();

    let mut current_dir = 0;

    let directions: Vec<Direction> = vec![
        Direction{x:0,y:-1},
        Direction{x:1,y:0},
        Direction{x:0,y:1},
        Direction{x:-1,y:0},
    ];

    loop {
        counter.insert((guard_location.x, guard_location.y));
        let dir = directions[current_dir]; 
        let next = Direction{x: guard_location.x + dir.x, y: guard_location.y + dir.y};
        let lookahead = grid.get(next.y, next.x);
        let block: Block;
        
        match lookahead {
           Some(b) => {block = *b}
           None => {break;} 
        }
        match block {
            Block::Obstacle => {current_dir = (current_dir + 1) % directions.len(); continue;}   
            Block::Empty => {
                guard_location = next;
            } 
        }

    }

    println!("{:?}", counter.len());

    return Ok(());
}

fn is_loop(guard: Direction, grid: Grid<Block>) -> bool {
    let mut guard_location = guard;
    let mut current_dir = 0;

    let directions: Vec<Direction> = vec![
        Direction{x:0,y:-1},
        Direction{x:1,y:0},
        Direction{x:0,y:1},
        Direction{x:-1,y:0},
    ];

    let mut i = 0;

    loop {
        i += 1;

        if i >= 10000 {
            return true;
        }

        let dir = directions[current_dir]; 
        let next = Direction{x: guard_location.x + dir.x, y: guard_location.y + dir.y};
        let lookahead = grid.get(next.y, next.x);


        let block: Block;
        
        match lookahead {
           Some(b) => {block = *b}
           None => {break;} 
        }

        match block {
            Block::Obstacle => {
                current_dir = (current_dir + 1) % directions.len(); 
                continue;
            }   
            Block::Empty => {
                guard_location = next;
            } 
        }

    }
    return false;
}

pub fn part2() -> std::io::Result<()> {
    let file = File::open("./src/inputs/6.txt")?;
    let reader = BufReader::new(file);

    let mut grid: Grid<Block> = grid![];
    let mut guard_location = Direction{x: 0, y: 0};

    for (i, line) in reader.lines().enumerate()  {
        let content = line.unwrap();
        let mut row: Vec<Block> = vec![];
        for (j, ch) in content.chars().enumerate() {
           match ch {
              '.' => {row.push(Block::Empty);}
              '#' => {row.push(Block::Obstacle);}
              '^' => {row.push(Block::Empty); guard_location = Direction{x: j as i32, y: i as i32};}
              _ => {} 
           } 
        }
        grid.push_row(row);
    }

    let mut counter = 0;

    for ((row, col), block) in grid.indexed_iter() {
        match block {
            Block::Empty => {}
            Block::Obstacle => {continue;}
        }
        let mut new_grid = grid.clone();
        new_grid[(row, col)] = Block::Obstacle;

        counter += if is_loop(guard_location, new_grid) {1} else {0};
    } 


    println!("{:?}", counter);

    return Ok(());
}
