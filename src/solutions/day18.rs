use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet, VecDeque};
use std::fs::File;
use std::hash::Hash;
use std::io::{BufRead, BufReader};
use grid::Grid;
use regex::Regex;
use std::usize;

#[derive(Debug, Clone, Copy, Default)]
enum Entry {
    Wall,
    #[default]
    Empty,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Cartesian {
    x: i64,
    y: i64,
}

impl Cartesian {
    fn to_node(&self, target: &Cartesian, pg: i64, dir: Direction) -> Node {
        let g = pg + 1;
        let h = heuristic(*self, *target);
        return Node{pos: *self, g, h, f: g + h, dir};
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
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

fn visualize(grid: &Grid<Entry>) {
    for (_y, row) in grid.iter_rows().enumerate() {
       for (_x, entry) in row.enumerate() {
            match entry {
                Entry::Wall => {print!("#")}
                Entry::Empty => {print!(".")}
            }
       } 
        println!("");
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
    open_list.push(start.to_node(&end, -1, Direction::Right));
    let mut closed_list: HashSet<Cartesian> = HashSet::new();
    while !open_list.is_empty() {
        let curr = open_list.pop().unwrap(); 
        closed_list.insert(curr.pos);
        let pos = curr.pos;
        if end == pos {
            return curr.g;
        }
        for (_i, d) in curr.dir.rotate().iter().enumerate() {
            let dir = d.value(); 
            let lookahead = Cartesian{x: pos.x + dir.0, y: pos.y + dir.1};

            match grid.get(lookahead.y, lookahead.x) {
                Some(entry) => {
                    match entry {
                        Entry::Empty => {
                            if !closed_list.contains(&lookahead) {
                                open_list.push(lookahead.to_node(&end, curr.g, *d));
                            }
                        }
                        Entry::Wall => {}
                    }
                }
                None => {}
            }
        }
    }
    return -1;
}


pub fn part1() -> std::io::Result<()> {
    let file = File::open("./src/inputs/18.txt")?;
    let reader = BufReader::new(file);

    let bounds = 71;
    let mut grid: Grid<Entry> = Grid::new(bounds as usize, bounds as usize);
    let start: Cartesian = Cartesian{x: 0, y: 0};
    let end: Cartesian = Cartesian{x: bounds - 1, y: bounds - 1};
    let rex = Regex::new(r"(?<x>[-]?\d+),(?<y>[-]?\d+)").unwrap();

    for (i, lines) in reader.lines().enumerate() {
        let content = lines.unwrap();
        if i == 1024 {
            break;
        }
        for c in rex.captures(&content).iter() {
            let x = &c["x"].parse::<i32>().unwrap();
            let y = &c["y"].parse::<i32>().unwrap();
            if let Some(entry) = grid.get_mut(*y, *x) {
                *entry = Entry::Wall;
            }
        }
    }

    println!("{:?}", astar(start, end, &grid));

    return Ok(());
}

pub fn part2() -> std::io::Result<()> {
    let file = File::open("./src/inputs/18.txt")?;
    let reader = BufReader::new(file);

    let bounds = 71;
    let mut grid: Grid<Entry> = Grid::new(bounds as usize, bounds as usize);
    let mut bytes: VecDeque<(usize, usize)> = VecDeque::new();
    let start: Cartesian = Cartesian{x: 0, y: 0};
    let end: Cartesian = Cartesian{x: bounds - 1, y: bounds - 1};
    let rex = Regex::new(r"(?<x>[-]?\d+),(?<y>[-]?\d+)").unwrap();

    for (i, lines) in reader.lines().enumerate() {
        let content = lines.unwrap();
        for c in rex.captures(&content).iter() {
            let x = c["x"].parse::<i32>().unwrap();
            let y = c["y"].parse::<i32>().unwrap();
            if i < 1024 {
                if let Some(entry) = grid.get_mut(y, x) {
                    *entry = Entry::Wall;
                }
            } else {
                bytes.push_back((x as usize, y as usize));
            }
        }
    }

    for b in bytes {
        if let Some(entry) = grid.get_mut(b.1, b.0) {
            *entry = Entry::Wall;
        }

        if astar(start, end, &grid) == -1 {
            println!("{:?},{:?}", b.0, b.1);
            break;
        }
    }

    return Ok(());
}
