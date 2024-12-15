use std::process::Command;
use std::{thread, time};
use std::{char, usize};
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use grid::{grid, Grid};

enum Entry {
    Wall,
    Box,
    Empty,
}

#[derive(Debug, Clone, Copy)]
struct Cartesian {
    x: i8,
    y: i8,
}

enum Direction {
    Up,
    Down,
    Right,
    Left,
}

impl Direction {
    fn value(&self) -> (i8, i8) {
        match *self {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Right => (1, 0),
            Direction::Left => (-1, 0),
        }
    }
}

fn visualize(grid: &Grid<Entry>, robot: &Cartesian) {
    for (y, row) in grid.iter_rows().enumerate() {
       for (x, entry) in row.enumerate() {
           if x == robot.x as usize && y == robot.y as usize { print!("@");} else {
                match entry {
                    Entry::Wall => {print!("#")}
                    Entry::Box => {print!("O")}
                    Entry::Empty => {print!(".")}
                }
            }
       } 
        println!("");
    }
}

fn push(direction: &Direction, coords: &Cartesian, grid: &mut Grid<Entry>) -> bool {
    let val = direction.value();
    let lookahead = Cartesian{y: coords.y as i8 + val.1, x: coords.x as i8 + val.0};
    match grid.get(lookahead.y, lookahead.x) {
        Some(e) => {
            match e {
                Entry::Wall => {
                    return false;
                } 
                Entry::Box => {
                    if push(direction, &lookahead, grid) {
                        grid.swap(((lookahead.y + val.1).try_into().unwrap(), (lookahead.x + val.0).try_into().unwrap()), (lookahead.y.try_into().unwrap(), lookahead.x.try_into().unwrap()));
                        return true;
                    } else {
                        return false;
                    }
                } 
                Entry::Empty => {
                    return true;
                } 
            }
        }
        None => {} 
    }
    return false;
}

fn move_robot(direction: &Direction, robot: &mut Cartesian, grid: &mut Grid<Entry>) {
    let val = direction.value();
    let lookahead = Cartesian{y: robot.y as i8 + val.1, x: robot.x as i8 + val.0};
    if push(direction, robot, grid) {
        *robot = lookahead;
    }
}

pub fn part1() -> std::io::Result<()> {
    let file = File::open("./src/inputs/15.txt")?;
    let reader = BufReader::new(file);

    let mut grid: Grid<Entry> = grid![];
    let mut robot: Cartesian = Cartesian{x: 0, y: 0};
    let mut directions: Vec<Direction> = vec![];

    let mut parse_field = true;

    for (i, lines) in reader.lines().enumerate() {
        let content = lines.unwrap();
        if content == "" {
            parse_field = false;
            continue;
        }
        if parse_field {
            let mut row: Vec<Entry> = vec![];
            for (j, e) in content.chars().enumerate() {
                match e {
                    '#' => {row.push(Entry::Wall);}
                    'O' => {row.push(Entry::Box);}
                    '.' => {row.push(Entry::Empty);}
                    '@' => {robot.x = j as i8; robot.y = i as i8; row.push(Entry::Empty);}
                    _ => {}
                } 
            }
            grid.push_row(row);
        } else {
            for d in content.chars() {
                match d {
                    '^' => {directions.push(Direction::Up);}
                    'v' => {directions.push(Direction::Down);}
                    '>' => {directions.push(Direction::Right);}
                    '<' => {directions.push(Direction::Left);}
                    _ => {}
                } 
            }
        }
    }

    for (i, dir) in directions.iter().enumerate() {
        move_robot(dir, &mut robot, &mut grid);    
    }

    let mut counter = 0;

    for ((y, x), entry) in grid.indexed_iter() {
       match entry {
        Entry::Box => {
                counter += (y * 100) + x;
            }    
        _ => {}
       } 
    } 

    println!("{:?}", counter);

    return Ok(());
}


#[derive(Debug, Clone, Copy)]
enum Entry2 {
    Wall,
    BoxLeft,
    BoxRight,
    Empty,
}

fn visualize2(grid: &Grid<Entry2>, robot: &Cartesian) {
    for (y, row) in grid.iter_rows().enumerate() {
       for (x, entry) in row.enumerate() {
           if x == robot.x as usize && y == robot.y as usize { print!("@");} else {
                match entry {
                    Entry2::Wall => {print!("#")}
                    Entry2::BoxLeft => {print!("[")}
                    Entry2::BoxRight => {print!("]")}
                    Entry2::Empty => {print!(".")}
                }
            }
       } 
        println!("");
    }
}

fn push2(direction: &Direction, coords: &Cartesian, grid: &mut Grid<Entry2>, ignore: bool) -> bool {
    let val = direction.value();
    let lookahead = Cartesian{y: coords.y as i8 + val.1, x: coords.x as i8 + val.0};
    match grid.get(lookahead.y, lookahead.x) {
        Some(e) => {
            match e {
                Entry2::Wall => {
                    return false;
                } 
                Entry2::BoxLeft => {
                    if push2(direction, &lookahead, grid, false) {
                        let right = Cartesian{x: coords.x + 1, y: coords.y};
                        if val.1 != 0 && !ignore {
                            if !push2(direction, &right, grid, true) {
                                return false;
                            }
                        }
                        return true;
                    } else {
                        return false;
                    }
                } 
                Entry2::BoxRight => {
                    if push2(direction, &lookahead, grid, false) {
                        let left = Cartesian{x: coords.x - 1, y: coords.y};
                        if val.1 != 0 && !ignore {
                            if !push2(direction, &left, grid, true) {
                                return false;
                            }
                        }
                        return true;
                    } else {
                        return false;
                    }
                } 
                Entry2::Empty => {
                    return true;
                } 
            }
        }
        None => {} 
    }
    return false;
}

fn push2move(direction: &Direction, coords: &Cartesian, grid: &mut Grid<Entry2>, ignore: bool) {
    let val = direction.value();
    let lookahead = Cartesian{y: coords.y + val.1, x: coords.x + val.0};
    match grid.get(lookahead.y, lookahead.x) {
        Some(e) => {
            match e {
                Entry2::BoxLeft => {
                    push2move(direction, &lookahead, grid, false);
                    let right = Cartesian{x: coords.x + 1, y: coords.y};
                    if val.1 != 0 && !ignore {
                        push2move(direction, &right, grid, true);
                    }
                    grid.swap(((lookahead.y + val.1).try_into().unwrap(), (lookahead.x + val.0).try_into().unwrap()), (lookahead.y.try_into().unwrap(), lookahead.x.try_into().unwrap()));
                } 
                Entry2::BoxRight => {
                    push2move(direction, &lookahead, grid, false);
                    let left = Cartesian{x: coords.x - 1, y: coords.y};
                    if val.1 != 0 && !ignore {
                        push2move(direction, &left, grid, true);
                    }
                    grid.swap(((lookahead.y + val.1).try_into().unwrap(), (lookahead.x + val.0).try_into().unwrap()), (lookahead.y.try_into().unwrap(), lookahead.x.try_into().unwrap()));
                } 
                Entry2::Wall | Entry2::Empty => {}
            }
        }
        None => {} 
    }
}

fn move_robot2(direction: &Direction, robot: &mut Cartesian, grid: &mut Grid<Entry2>) {
    let val = direction.value();
    let lookahead = Cartesian{y: robot.y as i8 + val.1, x: robot.x as i8 + val.0};
    if push2(direction, robot, grid, false) {
        push2move(direction, robot, grid, false);
        *robot = lookahead;
    }
}

pub fn part2() -> std::io::Result<()> {
    let file = File::open("./src/inputs/15.txt")?;
    let reader = BufReader::new(file);

    let mut grid: Grid<Entry2> = grid![];
    let mut robot: Cartesian = Cartesian{x: 0, y: 0};
    let mut directions: Vec<Direction> = vec![];

    let mut parse_field = true;

    for (i, lines) in reader.lines().enumerate() {
        let content = lines.unwrap();
        if content == "" {
            parse_field = false;
            continue;
        }
        if parse_field {
            let mut row: Vec<Entry2> = vec![];
            for (j, e) in content.chars().enumerate() {
                match e {
                    '#' => {row.push(Entry2::Wall); row.push(Entry2::Wall)}
                    'O' => {row.push(Entry2::BoxLeft); row.push(Entry2::BoxRight);}
                    '.' => {row.push(Entry2::Empty); row.push(Entry2::Empty);}
                    '@' => {robot.x = j as i8 * 2; robot.y = i as i8; row.push(Entry2::Empty); row.push(Entry2::Empty);}
                    _ => {}
                } 
            }
            grid.push_row(row);
        } else {
            for d in content.chars() {
                match d {
                    '^' => {directions.push(Direction::Up);}
                    'v' => {directions.push(Direction::Down);}
                    '>' => {directions.push(Direction::Right);}
                    '<' => {directions.push(Direction::Left);}
                    _ => {}
                } 
            }
        }
    }

    for (i, dir) in directions.iter().enumerate() {
        move_robot2(dir, &mut robot, &mut grid);    
    }

    let mut counter = 0;

    for ((y, x), entry) in grid.indexed_iter() {
       match entry {
        Entry2::BoxLeft => {
                counter += (y * 100) + x;
            }    
        _ => {}
       } 
    }

    println!("{:?}", counter);

    return Ok(());
}
