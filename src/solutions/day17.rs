use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::usize;

#[derive(Debug, Copy, Clone)]
struct Registers {
    a: u64,
    b: u64,
    c: u64,
}

#[derive(Debug, Copy, Clone)]
enum Opcode {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

#[derive(Debug, Copy, Clone)]
struct Operator {
    literal: u64,
    combo: u64,
    opcode: Opcode,
}

impl Operator {
    const fn new(opcode: u64, operand: u64, reg: Registers) -> Self {
        let mut op = Self {
            literal: 0,
            combo: 0,
            opcode: Opcode::Adv,
        };
        match opcode {
            1 => op.opcode = Opcode::Bxl,
            2 => op.opcode = Opcode::Bst,
            3 => op.opcode = Opcode::Jnz,
            4 => op.opcode = Opcode::Bxc,
            5 => op.opcode = Opcode::Out,
            6 => op.opcode = Opcode::Bdv,
            7 => op.opcode = Opcode::Cdv,
            _ => {}
        }

        match operand {
            1 => {
                op.literal = 1;
                op.combo = 1;
            }
            2 => {
                op.literal = 2;
                op.combo = 2;
            }
            3 => {
                op.literal = 3;
                op.combo = 3;
            }
            4 => {
                op.literal = 4;
                op.combo = reg.a;
            }
            5 => {
                op.literal = 5;
                op.combo = reg.b;
            }
            6 => {
                op.literal = 6;
                op.combo = reg.c;
            }
            7 => op.literal = 7,
            _ => {}
        }
        op
    }

    fn eval(&mut self, reg: &mut Registers) -> bool {
        match self.literal {
            4 => {
                self.combo = reg.a;
            }
            5 => {
                self.combo = reg.b;
            }
            6 => {
                self.combo = reg.c;
            }
            _ => {}
        }
        match self.opcode {
            Opcode::Adv => {
                reg.a >>= self.combo;
            }
            Opcode::Bxl => {
                reg.b ^= self.literal;
            }
            Opcode::Bst => {
                reg.b = self.combo % 8;
            }
            Opcode::Jnz => {
                return true;
            }
            Opcode::Bxc => {
                reg.b ^= reg.c;
            }
            Opcode::Out => {
                let out = self.combo % 8;
                print!("{out:?}");
            }
            Opcode::Bdv => {
                reg.b = reg.a >> self.combo;
            }
            Opcode::Cdv => {
                reg.c = reg.a >> self.combo;
            }
        }
        false
    }

    fn eval2(&mut self, reg: &mut Registers) -> (u64, bool) {
        match self.literal {
            4 => {
                self.combo = reg.a;
            }
            5 => {
                self.combo = reg.b;
            }
            6 => {
                self.combo = reg.c;
            }
            _ => {}
        }
        match self.opcode {
            Opcode::Adv => {
                reg.a >>= self.combo;
            }
            Opcode::Bxl => {
                reg.b ^= self.literal;
            }
            Opcode::Bst => {
                reg.b = self.combo % 8;
            }
            Opcode::Jnz => {
                return (10, true);
            }
            Opcode::Bxc => {
                reg.b ^= reg.c;
            }
            Opcode::Out => {
                let out = self.combo % 8;
                return (out, false);
            }
            Opcode::Bdv => {
                reg.b = reg.a >> self.combo;
            }
            Opcode::Cdv => {
                reg.c = reg.a >> self.combo;
            }
        }
        (10, false)
    }
}

fn full_eval(ops: &Vec<Operator>, reg: &mut Registers) -> Vec<u64> {
    let mut outs: Vec<u64> = vec![];
    let mut i = 0;
    while i < ops.len() {
        let mut op = ops[i];
        let (out, jumped) = op.eval2(reg);
        if jumped && reg.a != 0 {
            i = op.literal as usize;
        } else {
            i += 1;
        }

        if out != 10 {
            outs.push(out);
        }
    }
    outs
}

pub fn part1() -> std::io::Result<()> {
    let file = File::open("./src/inputs/17.txt")?;
    let reader = BufReader::new(file);

    let counter = 0;
    let rex = Regex::new(r"(?<reg>\d+)").unwrap();
    let mut reg_nums: Vec<u64> = vec![];
    let mut ops: Vec<Operator> = vec![];
    let mut parse_reg = true;
    let mut reg = Registers { a: 0, b: 0, c: 0 };
    let mut program = String::new();

    for line in reader.lines() {
        let content = line.unwrap();
        if content.is_empty() {
            parse_reg = false;
            continue;
        }

        if !parse_reg {
            reg = Registers {
                a: reg_nums[0],
                b: reg_nums[1],
                c: reg_nums[2],
            };
            let split = content.split_once(' ').unwrap();
            let numbers = split.1.split(',');
            let mut v: Vec<u64> = vec![];
            for n in numbers {
                let x = n.parse::<u64>().unwrap();
                v.push(x);
                if v.len() == 2 {
                    let op = Operator::new(v[0], v[1], reg);
                    ops.push(op);
                    v = vec![];
                }
                program += n;
            }
        }

        for c in &rex.captures(&content) {
            let x = &c["reg"].parse::<u64>().unwrap();
            reg_nums.push(*x);
        }
    }

    let mut i = 0;
    while i < ops.len() {
        let mut op = ops[i];
        let jumped = op.eval(&mut reg);
        if jumped && reg.a != 0 {
            i = op.literal as usize;
        } else {
            i += 1;
        }
    }
    println!("\n{reg:?}");

    Ok(())
}

pub fn part2() -> std::io::Result<()> {
    let file = File::open("./src/inputs/17.txt")?;
    let reader = BufReader::new(file);

    let mut ops: Vec<Operator> = vec![];
    let mut parse_reg = true;
    let mut program: Vec<u64> = vec![];
    let reg = Registers { a: 0, b: 0, c: 0 };

    for line in reader.lines() {
        let content = line.unwrap();
        if content.is_empty() {
            parse_reg = false;
            continue;
        }

        if !parse_reg {
            let split = content.split_once(' ').unwrap();
            let numbers = split.1.split(',');
            let mut v: Vec<u64> = vec![];
            for n in numbers {
                let x = n.parse::<u64>().unwrap();
                v.push(x);
                if v.len() == 2 {
                    let op = Operator::new(v[0], v[1], reg);
                    ops.push(op);
                    v = vec![];
                }
                program.push(x);
            }
        }
    }

    let mut candidates: Vec<u64> = vec![0];
    for depth in 0..program.len() {
        let mut new_c: Vec<u64> = vec![];
        for candidate in &mut candidates {
            let a = *candidate * 8;
            for b in 0..8 {
                let mut r = Registers {
                    a: a + b,
                    b: 0,
                    c: 0,
                };
                let outs = full_eval(&ops, &mut r);
                if outs == program[program.len() - 1 - depth..] {
                    new_c.push(a + b);
                }
            }
        }
        candidates = new_c;
    }

    println!("{:?}", candidates[0]);

    Ok(())
}
