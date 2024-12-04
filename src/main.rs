mod solutions;
mod utils;
use std::env;
use std::time::Instant;

use solutions::day1;
use solutions::day3;
use solutions::day4;

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
      _ => todo!()  
    }; 
    println!("{:?}", start.elapsed());
    return Ok(());
}
