use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::hash::Hash;
use std::io::{BufRead, BufReader};
use grid::{grid, Grid};
use itertools::Itertools;
use std::{cmp, path, usize, vec};

#[derive(Debug, Clone, Copy)]
enum Entry {
    Wall,
    Empty,
}

#[derive(Debug, Clone, Copy)]
struct Cartesian {
    x: i64,
    y: i64,
}

impl PartialEq for Cartesian {
    fn eq(&self, other: &Cartesian) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Cartesian {
    fn to_node(&self, target: &Cartesian, i: usize, pg: i64) -> Node {
        let mut g = if i > 0 {1001} else {1};
        g += pg;
        let h = heuristic(*self, *target);
        return Node{pos: *self, g, h, f: g + h};
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}


fn heuristic(x: Cartesian, y: Cartesian) -> i64 {
  return (y.x - x.x).abs() + (y.y - x.y).abs();
}

impl Direction {
    fn value(&self) -> (i64, i64) {
        match *self {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Right => (1, 0),
            Direction::Left => (-1, 0),
        }
    }
    fn rotate(&self) -> [Direction; 3] {
        match *self {
            Direction::Up => [Direction::Up, Direction::Right, Direction::Left],
            Direction::Down => [Direction::Down, Direction::Right, Direction::Left],
            Direction::Left => [Direction::Left, Direction::Up, Direction::Down],
            Direction::Right => [Direction::Right, Direction::Up, Direction::Down],
        }
    }
}

fn visualize(grid: &Grid<Entry>, path: Vec<Cartesian>) {
    for (y, row) in grid.iter_rows().enumerate() {
       for (x, entry) in row.enumerate() {
           if path.contains(&Cartesian{x: x.try_into().unwrap(), y: y.try_into().unwrap()}) { print!("@");} else {
                match entry {
                    Entry::Wall => {print!("#")}
                    Entry::Empty => {print!(".")}
                }
            }
       } 
        println!("");
    }
}

#[derive(Debug, Clone, Copy)]
struct Node {
    pos: Cartesian,
    f: i64,
    g: i64,
    h: i64,
}

fn astar(start: Cartesian, end: Cartesian, grid: &Grid<Entry>) -> i64 {
    let mut open_list: Vec<(Node, Direction)> = vec![(start.to_node(&end, 0, -1), Direction::Right)];
    let mut closed_list: HashSet<(i64, i64)> = HashSet::new();
    while !open_list.is_empty() {
        let curr = open_list.remove(0); 
        let key = (curr.0.pos.x, curr.0.pos.y);
        if closed_list.contains(&key) {
            continue;
        }
        closed_list.insert(key);
        let pos = curr.0.pos;
        if curr.0.h == 0 {
            return curr.0.g;
        }
        for (i, d) in curr.1.rotate().iter().enumerate() {
            let dir = d.value(); 
            let lookahead = Cartesian{x: pos.x + dir.0, y: pos.y + dir.1};

            match grid.get(lookahead.y, lookahead.x) {
                Some(entry) => {
                    match entry {
                        Entry::Empty => {
                            open_list.push((lookahead.to_node(&end, i, curr.0.g), *d));
                        }
                        Entry::Wall => {}
                    }
                }
                None => {}
            }
        }
        open_list.sort_by(|a,b| a.0.f.cmp(&b.0.f));
    }
    return -1;
}

fn astar2(start: Cartesian, end: Cartesian, grid: &Grid<Entry>) -> (Vec<Cartesian>, HashMap<(i64, i64), (i64, Direction)>) {
    let mut open_list: Vec<(Node, Direction, Vec<Cartesian>)> = vec![(start.to_node(&end, 0, -1), Direction::Right, vec![start])];
    let mut closed_list: HashSet<(i64, i64)> = HashSet::new();
    let mut eq_list: HashMap<(i64, i64), (i64, Direction)> = HashMap::new();
    while !open_list.is_empty() {
        let curr = open_list.remove(0); 
        let key = (curr.0.pos.x, curr.0.pos.y);
        let pos = curr.0.pos;

        if closed_list.contains(&key) {
            continue;
        }

        closed_list.insert(key);

        if pos == end {
            return (curr.2, eq_list);
        }

        for (i, d) in curr.1.rotate().iter().enumerate() {
            let dir = d.value(); 
            let lookahead = Cartesian{x: pos.x + dir.0, y: pos.y + dir.1};

            match grid.get(lookahead.y, lookahead.x) {
                Some(entry) => {
                    match entry {
                        Entry::Empty => {
                            let mut path = curr.2.clone();
                            path.push(lookahead);
                            open_list.push((lookahead.to_node(&end, i, curr.0.g), *d, path));
                            eq_list.insert(key, (curr.0.g, *d));
                        }
                        Entry::Wall => {}
                    }
                }
                None => {}
            }
        }
        open_list.sort_by(|a,b| a.0.f.cmp(&b.0.f));
    }
    return (vec![], HashMap::new());
}

pub fn part1() -> std::io::Result<()> {
    let file = File::open("./src/inputs/16.txt")?;
    let reader = BufReader::new(file);

    let mut grid: Grid<Entry> = grid![];
    let mut start: Cartesian = Cartesian{x: 0, y: 0};
    let mut end: Cartesian = Cartesian{x: 0, y: 0};

    for (i, lines) in reader.lines().enumerate() {
        let content = lines.unwrap();
        let mut row: Vec<Entry> = vec![];
        for (j, e) in content.chars().enumerate() {
            match e {
                '#' => {row.push(Entry::Wall);}
                '.' => {row.push(Entry::Empty);}
                'S' => {start.x = j as i64; start.y = i as i64; row.push(Entry::Empty);}
                'E' => {end.x = j as i64; end.y = i as i64; row.push(Entry::Empty);}
                _ => {}
            } 
        }
        grid.push_row(row);
    }

    let mut counter = astar(start, end, &grid);
    println!("{:?}", counter);

    return Ok(());
}

pub fn part2() -> std::io::Result<()> {
    let file = File::open("./src/inputs/16.txt")?;
    let reader = BufReader::new(file);

    let mut grid: Grid<Entry> = grid![];
    let mut start: Cartesian = Cartesian{x: 0, y: 0};
    let mut end: Cartesian = Cartesian{x: 0, y: 0};

    for (i, lines) in reader.lines().enumerate() {
        let content = lines.unwrap();
        let mut row: Vec<Entry> = vec![];
        for (j, e) in content.chars().enumerate() {
            match e {
                '#' => {row.push(Entry::Wall);}
                '.' => {row.push(Entry::Empty);}
                'S' => {start.x = j as i64; start.y = i as i64; row.push(Entry::Empty);}
                'E' => {end.x = j as i64; end.y = i as i64; row.push(Entry::Empty);}
                _ => {}
            } 
        }
        grid.push_row(row);
    }

    let mut counter = 0;
    let length = astar(start, end, &grid);
    let (path, _crap) = astar2(start, end, &grid);

    for ((row, col), entry) in grid.indexed_iter() {
        match entry {
            Entry::Wall => {} 
            Entry::Empty => {
                let p = Cartesian{x: col as i64, y: row as i64};
                let sp = astar(start, p, &grid);
                let pe = astar(p, end, &grid);
                let (_crap, cost) = astar2(p, end, &grid);
                if path.contains(&p) {
                    println!("{:?} {:?}", sp, pe);
                    println!("{:?}", cost.get(&(p.x, p.y)));
                }
                counter += if sp + pe == length {1} else {0};
            }
        }
        
    }

    println!("{:?}", counter);

    return Ok(());
}
