use regex::Regex;
use std::fs::File;
use std::i32;
use std::io::{BufRead, BufReader};

#[derive(Debug, Copy, Clone)]
struct Cartesian {
    x: i32,
    y: i32,
}

#[derive(Debug, Copy, Clone)]
struct Robot {
    p: Cartesian,
    v: Cartesian,
}

fn second(robot: &mut Robot, bounds: Cartesian) {
    robot.p.x = (robot.p.x + robot.v.x).rem_euclid(bounds.x);
    robot.p.y = (robot.p.y + robot.v.y).rem_euclid(bounds.y);
}

fn draw(robots: &Vec<Robot>, bounds: Cartesian) {
    for i in 0..bounds.y {
        for j in 0..bounds.x {
            let robot: Vec<&Robot> = robots.iter().filter(|r| r.p.x == j && r.p.y == i).collect();
            if !robot.is_empty() {
                print!("■");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn quads(robot: Robot, bounds: Cartesian, quadrants: &mut [i32; 4]) {
    let halfx = bounds.x / 2;
    let halfy = bounds.y / 2;

    if robot.p.x == halfx || robot.p.y == halfy {
        return;
    }

    if robot.p.x > halfx && robot.p.y < halfy {
        quadrants[1] += 1;
    } else if robot.p.x < halfx && robot.p.y > halfy {
        quadrants[2] += 1;
    } else if robot.p.x > halfx && robot.p.y > halfy {
        quadrants[3] += 1;
    } else {
        quadrants[0] += 1;
    }
}

pub fn part1() -> std::io::Result<()> {
    let file = File::open("./src/inputs/14.txt")?;
    let reader = BufReader::new(file);

    let mut counter = 1;
    let rexp =
        Regex::new(r"p[=](?<px>[-]?\d+),(?<py>[-]?\d+) v[=](?<vx>[-]?\d+),(?<vy>[-]?\d+)").unwrap();

    let mut robots: Vec<Robot> = vec![];
    let bounds: Cartesian = Cartesian { x: 101, y: 103 };

    for line in reader.lines() {
        let content = line.unwrap();
        for c in &rexp.captures(&content) {
            let px = &c["px"].parse::<i32>().unwrap();
            let py = &c["py"].parse::<i32>().unwrap();
            let vx = &c["vx"].parse::<i32>().unwrap();
            let vy = &c["vy"].parse::<i32>().unwrap();
            robots.push(Robot {
                p: Cartesian { x: *px, y: *py },
                v: Cartesian { x: *vx, y: *vy },
            });
        }
    }

    for _i in 0..100 {
        for robot in &mut robots {
            second(robot, bounds);
        }
    }

    let mut quadrants: [i32; 4] = [0; 4];

    for robot in &robots {
        quads(*robot, bounds, &mut quadrants);
    }

    for q in quadrants {
        if q == 0 {
            continue;
        }
        counter *= q;
    }

    println!("{counter:?}");

    Ok(())
}

pub fn part2() -> std::io::Result<()> {
    let file = File::open("./src/inputs/14.txt")?;
    let reader = BufReader::new(file);

    let rexp =
        Regex::new(r"p[=](?<px>[-]?\d+),(?<py>[-]?\d+) v[=](?<vx>[-]?\d+),(?<vy>[-]?\d+)").unwrap();

    let mut robots: Vec<Robot> = vec![];
    let bounds: Cartesian = Cartesian { x: 101, y: 103 };

    for line in reader.lines() {
        let content = line.unwrap();
        for c in &rexp.captures(&content) {
            let px = &c["px"].parse::<i32>().unwrap();
            let py = &c["py"].parse::<i32>().unwrap();
            let vx = &c["vx"].parse::<i32>().unwrap();
            let vy = &c["vy"].parse::<i32>().unwrap();
            robots.push(Robot {
                p: Cartesian { x: *px, y: *py },
                v: Cartesian { x: *vx, y: *vy },
            });
        }
    }

    let mut security = i32::MAX;
    let mut index = 0;

    for i in 0..bounds.x * bounds.y {
        let mut quadrants: [i32; 4] = [0; 4];
        for robot in &mut robots {
            second(robot, bounds);
            quads(*robot, bounds, &mut quadrants);
        }
        let factor = quadrants[0] * quadrants[1] * quadrants[2] * quadrants[3];
        if security > factor {
            security = factor;
            index = i + 1;
        }
    }

    println!("{index:?}");

    Ok(())
}
