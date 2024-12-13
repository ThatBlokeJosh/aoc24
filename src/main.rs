mod solutions;
mod utils;
use std::env;
use std::time::Instant;

use solutions::day1;
use solutions::day10;
use solutions::day11;
use solutions::day12;
use solutions::day13;
use solutions::day3;
use solutions::day4;
use solutions::day5;
use solutions::day6;
use solutions::day7;
use solutions::day8;
use solutions::day9;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let start = Instant::now();
    let _ = match args[1].as_str() {
      "1-1" => day1::part1(), 
      "1-2" => day1::part2(), 
      "3-1" => day3::part1(), 
      "3-2" => day3::part2(), 
      "4-1" => day4::part1(), 
      "4-2" => day4::part2(), 
      "5-1" => day5::part1(), 
      "5-2" => day5::part2(), 
      "6-1" => day6::part1(), 
      "6-2" => day6::part2(), 
      "7-1" => day7::part1(), 
      "7-2" => day7::part2(), 
      "8-1" => day8::part1(), 
      "8-2" => day8::part2(), 
      "9-1" => day9::part1(), 
      "9-2" => day9::part2(), 
      "10-1" => day10::part1(), 
      "10-2" => day10::part2(), 
      "11-1" => day11::part1(), 
      "11-2" => day11::part2(), 
      "12-1" => day12::part1(), 
      "12-2" => day12::part2(), 
      "13-1" => day13::part1(), 
      "13-2" => day13::part2(), 
      _ => todo!()  
    }; 
    println!("{:?}", start.elapsed());
    return Ok(());
}
