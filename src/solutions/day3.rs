use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn part1() -> std::io::Result<()> {
    let file = File::open("./src/inputs/3.txt")?;
    let reader = BufReader::new(file);
    let re = Regex::new(r"mul\((?<n1>\d+),(?<n2>\d+)\)").unwrap();

    let mut res = 0;

    for line in reader.lines() {
        let hay = line.unwrap();
        for c in re.captures_iter(&hay) {
            let x = &c["n1"].parse::<i32>().unwrap();
            let y = &c["n2"].parse::<i32>().unwrap();
            res += x * y;
        }
    }

    println!("{res:?}");

    Ok(())
}

pub fn part2() -> std::io::Result<()> {
    let file = File::open("./src/inputs/3.txt")?;
    let reader = BufReader::new(file);
    let re = Regex::new(r"mul\((?<n>\d+),(?<n2>\d+)\)|(?<dont>don't\(\))|(?<do>do\(\))").unwrap();

    let mut res = 0;

    let mut do_bool = true;

    for line in reader.lines() {
        let hay = line.unwrap();
        for c in re.captures_iter(&hay) {
            match (c.name("dont"), c.name("do"), c.name("n"), c.name("n2")) {
                (None, None, Some(a), Some(b)) => {
                    let x = a.as_str().parse::<i32>().unwrap();
                    let y = b.as_str().parse::<i32>().unwrap();
                    if do_bool {
                        res += x * y;
                    }
                }
                (Some(dont), None, None, None) => {
                    do_bool = false;
                }
                (None, Some(do_cap), None, None) => {
                    do_bool = true;
                }
                _ => {}
            };
        }
    }

    println!("{res:?}");

    Ok(())
}
