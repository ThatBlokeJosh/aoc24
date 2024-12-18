use grid::Grid;
use regex::Regex;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet, VecDeque};
use std::fs::File;
use std::hash::Hash;
use std::io::{BufRead, BufReader};
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
    const fn to_node(&self, target: &Self, pg: i64, dir: Direction) -> Node {
        let g = pg + 1;
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

fn visualize(grid: &Grid<Entry>) {
    for row in grid.iter_rows() {
        for entry in row {
            match entry {
                Entry::Wall => {
                    print!("#")
                }
                Entry::Empty => {
                    print!(".")
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
    open_list.push(start.to_node(&end, -1, Direction::Right));
    let mut closed_list: HashSet<Cartesian> = HashSet::new();
    while !open_list.is_empty() {
        let curr = open_list.pop().unwrap();
        closed_list.insert(curr.pos);
        let pos = curr.pos;
        if end == pos {
            return curr.g;
        }
        for d in &curr.dir.rotate() {
            let dir = d.value();
            let lookahead = Cartesian {
                x: pos.x + dir.0,
                y: pos.y + dir.1,
            };

            if let Some(entry) = grid.get(lookahead.y, lookahead.x) {
                match entry {
                    Entry::Empty => {
                        if !closed_list.contains(&lookahead) {
                            open_list.push(lookahead.to_node(&end, curr.g, *d));
                        }
                    }
                    Entry::Wall => {}
                }
            }
        }
    }
    -1
}

pub fn part1() -> std::io::Result<()> {
    let file = File::open("./src/inputs/18.txt")?;
    let reader = BufReader::new(file);

    let bounds = 71;
    let mut grid: Grid<Entry> = Grid::new(bounds as usize, bounds as usize);
    let start: Cartesian = Cartesian { x: 0, y: 0 };
    let end: Cartesian = Cartesian {
        x: bounds - 1,
        y: bounds - 1,
    };
    let rex = Regex::new(r"(?<x>[-]?\d+),(?<y>[-]?\d+)").unwrap();

    for (i, lines) in reader.lines().enumerate() {
        let content = lines.unwrap();
        if i == 1024 {
            break;
        }
        for c in &rex.captures(&content) {
            let x = &c["x"].parse::<i32>().unwrap();
            let y = &c["y"].parse::<i32>().unwrap();
            if let Some(entry) = grid.get_mut(*y, *x) {
                *entry = Entry::Wall;
            }
        }
    }

    println!("{:?}", astar(start, end, &grid));

    Ok(())
}

pub fn part2() -> std::io::Result<()> {
    let file = File::open("./src/inputs/18.txt")?;
    let reader = BufReader::new(file);

    let bounds = 71;
    let mut grid: Grid<Entry> = Grid::new(bounds as usize, bounds as usize);
    let mut bytes: VecDeque<(usize, usize)> = VecDeque::new();
    let start: Cartesian = Cartesian { x: 0, y: 0 };
    let end: Cartesian = Cartesian {
        x: bounds - 1,
        y: bounds - 1,
    };
    let rex = Regex::new(r"(?<x>[-]?\d+),(?<y>[-]?\d+)").unwrap();

    for (i, lines) in reader.lines().enumerate() {
        let content = lines.unwrap();
        for c in &rex.captures(&content) {
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

    Ok(())
}
