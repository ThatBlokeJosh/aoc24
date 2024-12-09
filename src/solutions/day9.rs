use std::collections::{HashSet, VecDeque};
use std::fs::{self};
use std::io::{BufRead, BufReader};
use std::usize;

#[derive(Debug, Copy, Clone)]
enum Block {
    Data(u64),
    Empty, 
}

pub fn part1() -> std::io::Result<()> {
    let line = fs::read_to_string("./src/inputs/9.txt").unwrap();
    let disk = line.trim().chars().map(|n| n.to_digit(10).unwrap());

    let mut memory: Vec<Block> = vec![];
    let mut current_pos: u64 = 0;
    let mut queue: VecDeque<usize> = VecDeque::new();

    for (i, d) in disk.enumerate() {
        if (i % 2 == 0) {
            for _j in 0..d {
               memory.push(Block::Data(current_pos)); 
            }
            current_pos += 1;
        } else {
            for _j in 0..d {
               memory.push(Block::Empty); 
               queue.push_back(memory.len() - 1);
            }
        }
    }

    loop {
        let last = memory.pop().unwrap();    
        match last {
            Block::Empty => {continue;}
            _ => {}
        }
        match queue.get(0) {
            Some(i) => {
                if *i >= memory.len() {
                    memory.push(last);
                    break;
                }
                memory[*i] = last;
            }        
            None => {break;}
        }
        queue.pop_front();
    }

    let mut counter = 0;

    for (i, d) in memory.iter().enumerate() {
        match d {
            Block::Data(n) => {counter += i as u64 * n}
            _ => {} 
        }
    }

    println!("{:?}", counter);

    return Ok(());
}

#[derive(Debug, Copy, Clone)]
struct File {
    length: u64,
    block: Block,
    pos: usize,
}


#[derive(Debug, Copy, Clone)]
struct Empty {
    length: u64,
    pos: usize,
}

pub fn part2() -> std::io::Result<()> {
    let line = fs::read_to_string("./src/inputs/9.txt").unwrap();
    let disk = line.trim().chars().map(|n| n.to_digit(10).unwrap());

    let mut memory: Vec<Block> = vec![];
    let mut files: Vec<File> = vec![];
    let mut current_pos: u64 = 0;
    let mut queue: Vec<Empty> = Vec::new();

    for (i, d) in disk.enumerate() {
        if i % 2 == 0 {
            let f = File{length: d as u64, block: Block::Data(current_pos), pos: memory.len()};
            files.push(f);
            for _j in 0..d {
               memory.push(Block::Data(current_pos)); 
            }
            current_pos += 1;
        } else {
            let f = Empty{length: d as u64, pos: memory.len()};
            for _j in 0..d {
               memory.push(Block::Empty); 
            }
            queue.push(f);
        }
    }

    let mut counter = 0;

    'a: loop {
        let last;
        match files.pop() {
            Some(l) => {last = l;} 
            None => {break;}
        }
       
        for space in queue.iter_mut() {
            if (space.pos > last.pos) {
                break;
            }
            if space.length >= last.length {
                let mut new_file = last;
                new_file.pos = space.pos;
                space.length -= last.length;
                space.pos += last.length as usize;

                for i in 0..new_file.length {
                    memory[new_file.pos + i as usize] = new_file.block; 
                    memory[last.pos + i as usize] = Block::Empty;
                }

                break;
            } 
        }
    }



    for (i, d) in memory.iter().enumerate() {
        match d {
            Block::Data(n) => {counter += i as u64 * n}
            _ => {} 
        }
    }

    println!("{:?}", counter);

    return Ok(());
}
