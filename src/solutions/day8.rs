use grid::{grid, Grid};
use std::char;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Copy, Clone)]
struct Coordinate {
    x: i32,
    y: i32,
}

#[derive(Debug, Copy, Clone)]
enum Field {
    Antenna(char),
    Empty,
}

pub fn part1() -> std::io::Result<()> {
    let file = File::open("./src/inputs/8.txt")?;
    let reader = BufReader::new(file);

    let mut grid: Grid<Field> = grid![];
    let mut counter: HashSet<(i32, i32)> = HashSet::new();
    let mut antennas: HashMap<char, Vec<Coordinate>> = HashMap::new();

    for (i, lines) in reader.lines().enumerate() {
        let content = lines.unwrap();
        let fields: Vec<char> = content.chars().collect();
        let mut row: Vec<Field> = vec![];
        for (j, field) in fields.iter().enumerate() {
            if field == &'.' {
                row.push(Field::Empty);
            } else {
                row.push(Field::Antenna(*field));
                match antennas.get_mut(field) {
                    Some(v) => {
                        v.push(Coordinate {
                            x: j as i32,
                            y: i as i32,
                        });
                    }
                    None => {
                        antennas.insert(
                            *field,
                            vec![Coordinate {
                                x: j as i32,
                                y: i as i32,
                            }],
                        );
                    }
                }
            }
        }
        grid.push_row(row);
    }

    for (l, coords) in antennas {
        for (i, antenna) in coords.iter().enumerate() {
            for j in i + 1..coords.len() {
                let pair = coords[j];

                let diff1 = Coordinate {
                    x: antenna.x + (antenna.x - pair.x),
                    y: antenna.y + (antenna.y - pair.y),
                };

                if let Some(_b) = grid.get(diff1.y, diff1.x) {
                    counter.insert((diff1.x, diff1.y));
                }

                let diff2 = Coordinate {
                    x: pair.x + (pair.x - antenna.x),
                    y: pair.y + (pair.y - antenna.y),
                };

                if let Some(_b) = grid.get(diff2.y, diff2.x) {
                    counter.insert((diff2.x, diff2.y));
                }
            }
        }
    }

    println!("{:?}", counter.len());

    Ok(())
}

pub fn part2() -> std::io::Result<()> {
    let file = File::open("./src/inputs/8.txt")?;
    let reader = BufReader::new(file);

    let mut grid: Grid<Field> = grid![];
    let mut counter: HashSet<(i32, i32)> = HashSet::new();
    let mut antennas: HashMap<char, Vec<Coordinate>> = HashMap::new();

    for (i, lines) in reader.lines().enumerate() {
        let content = lines.unwrap();
        let fields: Vec<char> = content.chars().collect();
        let mut row: Vec<Field> = vec![];
        for (j, field) in fields.iter().enumerate() {
            if field == &'.' {
                row.push(Field::Empty);
            } else {
                row.push(Field::Antenna(*field));
                match antennas.get_mut(field) {
                    Some(v) => {
                        v.push(Coordinate {
                            x: j as i32,
                            y: i as i32,
                        });
                    }
                    None => {
                        antennas.insert(
                            *field,
                            vec![Coordinate {
                                x: j as i32,
                                y: i as i32,
                            }],
                        );
                    }
                }
            }
        }
        grid.push_row(row);
    }

    for (l, coords) in antennas {
        for (i, antenna) in coords.iter().enumerate() {
            counter.insert((antenna.x, antenna.y));
            for j in i + 1..coords.len() {
                let pair = coords[j];

                let diff1 = Coordinate {
                    x: antenna.x - pair.x,
                    y: antenna.y - pair.y,
                };
                let mut t = *antenna;

                'a: loop {
                    let antinode = Coordinate {
                        x: t.x + diff1.x,
                        y: t.y + diff1.y,
                    };
                    match grid.get(antinode.y, antinode.x) {
                        Some(_b) => {
                            counter.insert((antinode.x, antinode.y));
                        }
                        None => {
                            break 'a;
                        }
                    }
                    t = antinode;
                }

                let diff2 = Coordinate {
                    x: pair.x - antenna.x,
                    y: pair.y - antenna.y,
                };
                let mut t2 = pair;

                'b: loop {
                    let antinode = Coordinate {
                        x: t2.x + diff2.x,
                        y: t2.y + diff2.y,
                    };
                    match grid.get(antinode.y, antinode.x) {
                        Some(_b) => {
                            counter.insert((antinode.x, antinode.y));
                        }
                        None => {
                            break 'b;
                        }
                    }
                    t2 = antinode;
                }
            }
        }
    }

    println!("{:?}", counter.len());

    Ok(())
}
