use core::time;
use std::{time::{Duration, Instant}, usize};
use crate::solutions::*;

use textplots::{Chart, ColorPlot, Plot, Shape};
use rgb::RGB8;

use crate::solutions;

fn match_day(day: usize) -> Option<Duration> {
    let start = Instant::now();
    let _ = match day {
      0 => day1::part1(), 
      1 => day1::part2(), 
      2 => day3::part1(), 
      3 => day3::part2(), 
      4 => day4::part1(), 
      5 => day4::part2(), 
      6 => day5::part1(), 
      7 => day5::part2(), 
      10 => day7::part1(), 
      11 => day7::part2(), 
      12 => day8::part1(), 
      13 => day8::part2(), 
      14 => day9::part1(), 
      15 => day9::part2(), 
      16 => day10::part1(), 
      17 => day10::part2(), 
      18 => day11::part1(), 
      19 => day11::part2(), 
      20 => day12::part1(), 
      21 => day12::part2(), 
      22 => day13::part1(), 
      23 => day13::part2(), 
      _ => {return None;}  
    }; 
    return Some(start.elapsed());
}

pub fn draw() {
  let mut points: Vec<(f32, f32)> = vec![];
  for day in 0..50 {
    if let Some(duration) = match_day(day) {
      points.push((day as f32, duration.as_micros() as f32));
    }
  }
  println!("\n{:-^151}", "Times");
  Chart::new(300, 100, 0.0, 50.0)
    .linecolorplot(&Shape::Bars(&points), RGB8{r: 0, g: 255, b: 0})
    .nice();

}
