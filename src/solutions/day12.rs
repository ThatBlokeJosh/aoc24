use grid::{grid, Grid};
use once_cell::sync::Lazy;
use std::char;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Clone, Copy)]
struct Plot {
    letter: char,
}

#[derive(Debug, Clone, Copy)]
struct Sides {
    top: *mut Node,
    bottom: *mut Node,
    left: *mut Node,
    right: *mut Node,
}

#[derive(Debug, Clone, Copy)]
struct Node {
    letter: char,
    visited: bool,
    sides: [*mut Node; 4],
}

static DIRS: Lazy<[(i32, i32); 4]> = Lazy::new(|| [(1, 0), (-1, 0), (0, 1), (0, -1)]);

fn descent(
    plot: Plot,
    row: i32,
    col: i32,
    grid: &mut Grid<Plot>,
    counter: &mut u64,
    visited: &mut HashSet<(i32, i32)>,
) -> u64 {
    visited.insert((row, col));
    let mut empty = 4;
    let mut length = 1;
    for dir in DIRS.iter() {
        let new_pos: (i32, i32) = (row + dir.0, col + dir.1);
        if let Some(p) = grid.get(new_pos.0, new_pos.1) {
            if p.letter == plot.letter {
                if let Some(_n) = visited.get(&new_pos) {
                    empty -= 1;
                    continue;
                }
                empty -= 1;
                length += descent(plot, new_pos.0, new_pos.1, grid, counter, visited);
            }
        }
    }
    *counter += empty;
    length
}

pub fn part1() -> std::io::Result<()> {
    let file = File::open("./src/inputs/12.txt")?;
    let reader = BufReader::new(file);

    let mut grid: Grid<Plot> = grid![];

    for line in reader.lines() {
        let content = line.unwrap();
        let row: Vec<Plot> = content.chars().map(|p| Plot { letter: p }).collect();
        grid.push_row(row);
    }

    let mut seen: HashSet<(i32, i32)> = HashSet::new();
    let mut counter = 0;

    for ((row, col), plot) in grid.clone().indexed_iter() {
        let mut perimiter = 0;
        if let Some(_n) = seen.get(&(row as i32, col as i32)) {
            continue;
        }
        let area = descent(
            *plot,
            row as i32,
            col as i32,
            &mut grid,
            &mut perimiter,
            &mut seen,
        );
        counter += area * perimiter;
    }

    println!("{counter:?}");

    Ok(())
}

static DIAGS: Lazy<[(i32, i32); 4]> = Lazy::new(|| [(1, 1), (-1, -1), (-1, 1), (1, -1)]);

fn descent2(
    plot: Plot,
    row: i32,
    col: i32,
    grid: &mut Grid<Plot>,
    counter: &mut u64,
    visited: &mut HashSet<(i32, i32)>,
) -> u64 {
    visited.insert((row, col));
    let mut length = 1;
    let mut neighbors: [bool; 4] = [false; 4];
    for (i, dir) in DIRS.iter().enumerate() {
        let new_pos: (i32, i32) = (row + dir.0, col + dir.1);
        if let Some(p) = grid.get(new_pos.0, new_pos.1) {
            if p.letter == plot.letter {
                neighbors[i] = true;
                if let Some(_n) = visited.get(&new_pos) {
                    continue;
                }
                length += descent2(plot, new_pos.0, new_pos.1, grid, counter, visited);
            }
        }
    }

    let mut dia: [bool; 4] = [false; 4];
    for (i, dir) in DIAGS.iter().enumerate() {
        let new_pos: (i32, i32) = (row + dir.0, col + dir.1);
        if let Some(p) = grid.get(new_pos.0, new_pos.1) {
            if p.letter == plot.letter {
                dia[i] = true;
            }
        }
    }

    if !neighbors[0] && !neighbors[2] {
        *counter += 1
    }
    if !neighbors[0] && !neighbors[3] {
        *counter += 1
    }
    if !neighbors[1] && !neighbors[2] {
        *counter += 1
    }
    if !neighbors[1] && !neighbors[3] {
        *counter += 1
    }

    if !dia[0] && neighbors[0] && neighbors[2] {
        *counter += 1
    }
    if !dia[3] && neighbors[0] && neighbors[3] {
        *counter += 1
    }
    if !dia[2] && neighbors[1] && neighbors[2] {
        *counter += 1
    }
    if !dia[1] && neighbors[1] && neighbors[3] {
        *counter += 1
    }

    length
}

pub fn part2() -> std::io::Result<()> {
    let file = File::open("./src/inputs/12.txt")?;
    let reader = BufReader::new(file);

    let mut grid: Grid<Plot> = grid![];

    for line in reader.lines() {
        let content = line.unwrap();
        let row: Vec<Plot> = content.chars().map(|p| Plot { letter: p }).collect();
        grid.push_row(row);
    }

    let seen: HashSet<(i32, i32)> = HashSet::new();

    let mut seen: HashSet<(i32, i32)> = HashSet::new();
    let mut counter = 0;

    for ((row, col), plot) in grid.clone().indexed_iter() {
        let mut perimiter = 0;
        if let Some(_n) = seen.get(&(row as i32, col as i32)) {
            continue;
        }
        let area = descent2(
            *plot,
            row as i32,
            col as i32,
            &mut grid,
            &mut perimiter,
            &mut seen,
        );
        counter += area * perimiter;
    }

    println!("{counter:?}");

    Ok(())
}
