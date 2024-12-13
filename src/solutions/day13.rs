use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;

#[derive(Debug, Copy, Clone)]
struct Matrix {
    x: f64,
    y: f64,
    z: f64,
}

fn gauss(eq_1: Matrix, eq_2: Matrix) -> (f64, f64) {
    let mult_1 = Matrix{x: eq_1.x * eq_2.x, y: eq_1.y * eq_2.x, z: eq_1.z * eq_2.x}; 
    let mult = eq_1.x * -1.0;
    let mult_2 = Matrix{x: eq_2.x * mult, y: eq_2.y * mult, z: eq_2.z * mult}; 
    let eq_3 = Matrix{x: mult_1.x + mult_2.x, y: mult_1.y + mult_2.y, z: mult_1.z + mult_2.z};
    let y = eq_3.z / eq_3.y;
    let x = (eq_1.z - (y*eq_1.y)) / eq_1.x;
    if (x % 1.0, y % 1.0) != (0.0, 0.0) {
        return (0.0, 0.0);
    }
    return (x, y);
}

pub fn part1() -> std::io::Result<()> {
    let file = File::open("./src/inputs/13.txt")?;
    let reader = BufReader::new(file);

    let mut counter = 0;
    let rex = Regex::new(r"X[+=](?<x>\d+), Y[+=](?<y>\d+)").unwrap();

    let empty = Matrix{x: 0.0, y: 0.0, z: 0.0};
    let mut eq_1 = Matrix{x: 0.0, y: 0.0, z: 0.0};
    let mut eq_2 = Matrix{x: 0.0, y: 0.0, z: 0.0};

    for line in reader.lines()  {
        let content = line.unwrap();
        for c in rex.captures(&content).iter() {
            let x = &c["x"].parse::<f64>().unwrap();
            let y = &c["y"].parse::<f64>().unwrap();
            if eq_1.x == 0.0 {
                eq_1.x = *x;
                eq_2.x = *y;
            } else if eq_1.y == 0.0 {
                eq_1.y = *x;
                eq_2.y = *y;
            } else {
                eq_1.z = *x;
                eq_2.z = *y;
                let ab = gauss(eq_1, eq_2);
                counter += ((ab.0 * 3.) + (ab.1)) as i32;
                eq_1 = empty;
                eq_2 = empty;
            }
        }
    }

    println!("{:?}", counter);

    return Ok(());
}

pub fn part2() -> std::io::Result<()> {
    let file = File::open("./src/inputs/13.txt")?;
    let reader = BufReader::new(file);

    let mut counter = 0;
    let rex = Regex::new(r"X[+=](?<x>\d+), Y[+=](?<y>\d+)").unwrap();

    let empty = Matrix{x: 0.0, y: 0.0, z: 0.0};
    let mut eq_1 = Matrix{x: 0.0, y: 0.0, z: 0.0};
    let mut eq_2 = Matrix{x: 0.0, y: 0.0, z: 0.0};

    for line in reader.lines()  {
        let content = line.unwrap();
        for c in rex.captures(&content).iter() {
            let x = &c["x"].parse::<f64>().unwrap();
            let y = &c["y"].parse::<f64>().unwrap();
            if eq_1.x == 0.0 {
                eq_1.x = *x;
                eq_2.x = *y;
            } else if eq_1.y == 0.0 {
                eq_1.y = *x;
                eq_2.y = *y;
            } else {
                eq_1.z = *x + 10000000000000.0;
                eq_2.z = *y + 10000000000000.0;
                let ab = gauss(eq_1, eq_2);
                counter += ((ab.0 * 3.) + (ab.1)) as i64;
                eq_1 = empty;
                eq_2 = empty;
            }
        }
    }

    println!("{:?}", counter);
    return Ok(());
}
