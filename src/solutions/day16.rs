use grid::{grid, Grid};
use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fs::File;
use std::hash::Hash;
use std::io::{BufRead, BufReader};
use std::{usize, vec};

#[derive(Debug, Clone, Copy)]
enum Entry {
    Wall,
    Empty,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Cartesian {
    x: i64,
    y: i64,
}

impl Cartesian {
    const fn to_node(&self, target: &Self, i: usize, pg: i64, dir: Direction) -> Node {
        let mut g = if i > 0 { 1001 } else { 1 };
        g += pg;
        let h = heuristic(*self, *target);
        Node {
            pos: *self,
            g,
            h,
            f: g + h,
            dir,
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

const fn heuristic(x: Cartesian, y: Cartesian) -> i64 {
    (y.x - x.x).abs() + (y.y - x.y).abs()
}

impl Direction {
    const fn value(&self) -> (i64, i64) {
        match *self {
            Self::Up => (0, -1),
            Self::Down => (0, 1),
            Self::Right => (1, 0),
            Self::Left => (-1, 0),
        }
    }
    const fn rotate(&self) -> [Self; 3] {
        match *self {
            Self::Up => [Self::Up, Self::Right, Self::Left],
            Self::Down => [Self::Down, Self::Right, Self::Left],
            Self::Left => [Self::Left, Self::Up, Self::Down],
            Self::Right => [Self::Right, Self::Up, Self::Down],
        }
    }
}

fn visualize(grid: &Grid<Entry>, path: Vec<Cartesian>) {
    for (y, row) in grid.iter_rows().enumerate() {
        for (x, entry) in row.enumerate() {
            if path.contains(&Cartesian {
                x: x.try_into().unwrap(),
                y: y.try_into().unwrap(),
            }) {
                print!("@");
            } else {
                match entry {
                    Entry::Wall => {
                        print!("#")
                    }
                    Entry::Empty => {
                        print!(".")
                    }
                }
            }
        }
        println!();
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Node {
    pos: Cartesian,
    dir: Direction,
    f: i64,
    g: i64,
    h: i64,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.f.cmp(&other.f)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(other.f.cmp(&self.f))
    }
}

fn astar(start: Cartesian, end: Cartesian, grid: &Grid<Entry>) -> i64 {
    let mut open_list: BinaryHeap<Node> = BinaryHeap::new();
    open_list.push(start.to_node(&end, 0, -1, Direction::Right));
    let mut closed_list: HashSet<Cartesian> = HashSet::new();
    while !open_list.is_empty() {
        let curr = open_list.pop().unwrap();
        closed_list.insert(curr.pos);
        let pos = curr.pos;
        if curr.h == 0 {
            return curr.g;
        }
        for (i, d) in curr.dir.rotate().iter().enumerate() {
            let dir = d.value();
            let lookahead = Cartesian {
                x: pos.x + dir.0,
                y: pos.y + dir.1,
            };

            if let Some(entry) = grid.get(lookahead.y, lookahead.x) {
                match entry {
                    Entry::Empty => {
                        if !closed_list.contains(&lookahead) {
                            open_list.push(lookahead.to_node(&end, i, curr.g, *d));
                        }
                    }
                    Entry::Wall => {}
                }
            }
        }
    }
    -1
}

fn astar2(start: Cartesian, end: Cartesian, grid: &Grid<Entry>) -> i64 {
    let mut open_list: BinaryHeap<Node> = BinaryHeap::new();
    open_list.push(start.to_node(&end, 0, -1, Direction::Right));
    let mut lowest_cost: HashMap<Cartesian, i64> = HashMap::new();
    let mut best_cost = i64::MAX;
    let mut paths: HashMap<Node, Vec<(Node, i64)>> = HashMap::new();
    paths.insert(
        *open_list.peek().unwrap(),
        vec![(*open_list.peek().unwrap(), 0)],
    );

    while !open_list.is_empty() {
        let curr = open_list.pop().unwrap();
        let pos = curr.pos;

        if lowest_cost.contains_key(&pos) && *lowest_cost.get(&pos).unwrap() < curr.g {
            continue;
        }
        lowest_cost.insert(pos, curr.g);

        if pos == end {
            if curr.g > best_cost {
                break;
            }
            best_cost = curr.g;
            let mut counter = 0;
            for point in paths.get(&curr).unwrap() {
                let paths_to_p = paths.get(&point.0).unwrap();
                for pp in paths_to_p {
                    if pp.1 + point.1 == best_cost {
                        counter += 1;
                    }
                }
            }
            println!("{counter:?}");
        }
        for (i, d) in curr.dir.rotate().iter().enumerate() {
            let dir = d.value();
            let lookahead = Cartesian {
                x: pos.x + dir.0,
                y: pos.y + dir.1,
            };

            if let Some(entry) = grid.get(lookahead.y, lookahead.x) {
                match entry {
                    Entry::Empty => {
                        let node = lookahead.to_node(&end, i, curr.g, *d);
                        let mut prev_path = paths.get(&curr).unwrap().clone();
                        prev_path.push((node, curr.g));
                        paths.insert(node, prev_path.clone());
                        open_list.push(node);
                    }
                    Entry::Wall => {}
                }
            }
        }
    }
    -1
}

pub fn part1() -> std::io::Result<()> {
    let file = File::open("./src/inputs/16.txt")?;
    let reader = BufReader::new(file);

    let mut grid: Grid<Entry> = grid![];
    let mut start: Cartesian = Cartesian { x: 0, y: 0 };
    let mut end: Cartesian = Cartesian { x: 0, y: 0 };

    for (i, lines) in reader.lines().enumerate() {
        let content = lines.unwrap();
        let mut row: Vec<Entry> = vec![];
        for (j, e) in content.chars().enumerate() {
            match e {
                '#' => {
                    row.push(Entry::Wall);
                }
                '.' => {
                    row.push(Entry::Empty);
                }
                'S' => {
                    start.x = j as i64;
                    start.y = i as i64;
                    row.push(Entry::Empty);
                }
                'E' => {
                    end.x = j as i64;
                    end.y = i as i64;
                    row.push(Entry::Empty);
                }
                _ => {}
            }
        }
        grid.push_row(row);
    }

    let counter = astar(start, end, &grid);
    println!("{counter:?}");

    Ok(())
}

pub fn part2() -> std::io::Result<()> {
    let file = File::open("./src/inputs/16.txt")?;
    let reader = BufReader::new(file);

    let mut grid: Grid<Entry> = grid![];
    let mut start: Cartesian = Cartesian { x: 0, y: 0 };
    let mut end: Cartesian = Cartesian { x: 0, y: 0 };

    for (i, lines) in reader.lines().enumerate() {
        let content = lines.unwrap();
        let mut row: Vec<Entry> = vec![];
        for (j, e) in content.chars().enumerate() {
            match e {
                '#' => {
                    row.push(Entry::Wall);
                }
                '.' => {
                    row.push(Entry::Empty);
                }
                'S' => {
                    start.x = j as i64;
                    start.y = i as i64;
                    row.push(Entry::Empty);
                }
                'E' => {
                    end.x = j as i64;
                    end.y = i as i64;
                    row.push(Entry::Empty);
                }
                _ => {}
            }
        }
        grid.push_row(row);
    }

    let mut counter = 0;
    let length = astar(start, end, &grid);

    for ((row, col), entry) in grid.indexed_iter() {
        match entry {
            Entry::Wall => {}
            Entry::Empty => {
                let p = Cartesian {
                    x: col as i64,
                    y: row as i64,
                };
                let sp = astar(start, p, &grid);
                let pe = astar(end, p, &grid);
                if sp + pe == length || sp + pe - 1000 == length {
                    counter += 1;
                }
            }
        }
    }

    println!("{counter:?}");

    Ok(())
}
