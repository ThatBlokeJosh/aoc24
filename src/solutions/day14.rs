use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;

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
            if robot.len() > 0 {
                print!("■");
            } else {
                print!(".");
            }
       }
       println!("");
    }

}

fn is_tree(robots: Vec<Robot>) -> bool {
    let mut seen: HashMap<i32, i32> = HashMap::new();
    for robot in robots {
        match seen.get_mut(&robot.p.y) {
            Some(y) => {
                *y += 1;
            }
            None => {
                seen.insert(robot.p.y, 1);
            } 
        }
    }
    for (y, count) in seen {
       if y + 1 != count {
            return false;
       } 
    }
    return true;
}

pub fn part1() -> std::io::Result<()> {
    let file = File::open("./src/inputs/14.txt")?;
    let reader = BufReader::new(file);

    let mut counter = 1;
    let rexp = Regex::new(r"p[=](?<px>[-]?\d+),(?<py>[-]?\d+) v[=](?<vx>[-]?\d+),(?<vy>[-]?\d+)").unwrap();

    let mut robots: Vec<Robot> = vec![];
    let bounds: Cartesian = Cartesian{x: 101, y: 103};

    for line in reader.lines()  {
        let content = line.unwrap();
        for c in rexp.captures(&content).iter() {
            let px = &c["px"].parse::<i32>().unwrap();
            let py = &c["py"].parse::<i32>().unwrap();
            let vx = &c["vx"].parse::<i32>().unwrap();
            let vy = &c["vy"].parse::<i32>().unwrap();
            robots.push(Robot{p: Cartesian{x: *px, y: *py}, v: Cartesian{x: *vx, y: *vy}});
            
        }
    }

    for _i in 0..100 {
        for robot in robots.iter_mut() {
           second(robot, bounds); 
        }
    }

    let mut quadrants: [i32; 4] = [0; 4];

    for robot in &robots {
        let halfx = bounds.x/2;
        let halfy = bounds.y/2;

        if robot.p.x == halfx || robot.p.y == halfy {
            continue;
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

    for q in quadrants {
        if q == 0 {
            continue;
        }
        counter *= q;
    }

    println!("{:?}", counter);

    return Ok(());
}

pub fn part2() -> std::io::Result<()> {
    let file = File::open("./src/inputs/14.txt")?;
    let reader = BufReader::new(file);

    let mut counter = 1;
    let rexp = Regex::new(r"p[=](?<px>[-]?\d+),(?<py>[-]?\d+) v[=](?<vx>[-]?\d+),(?<vy>[-]?\d+)").unwrap();

    let mut robots: Vec<Robot> = vec![];
    let bounds: Cartesian = Cartesian{x: 101, y: 103};

    for line in reader.lines()  {
        let content = line.unwrap();
        for c in rexp.captures(&content).iter() {
            let px = &c["px"].parse::<i32>().unwrap();
            let py = &c["py"].parse::<i32>().unwrap();
            let vx = &c["vx"].parse::<i32>().unwrap();
            let vy = &c["vy"].parse::<i32>().unwrap();
            robots.push(Robot{p: Cartesian{x: *px, y: *py}, v: Cartesian{x: *vx, y: *vy}});
            
        }
    }

    for i in 0..10000 {
        println!("\n{:?}\n", i);
        draw(&robots, bounds);
        for robot in robots.iter_mut() {
           second(robot, bounds); 
        }
    }

    return Ok(());
}
