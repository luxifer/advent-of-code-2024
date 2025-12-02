use anyhow::Result;
use common::cli;
use itertools::Itertools;
use regex::Regex;
use std::fmt;

fn main() -> Result<()> {
    let app = cli::Cli::new();
    let lines = app.content()?;

    let parts: Vec<&str> = lines.split("\n\n").collect();

    let re = Regex::new(r"Register (A|B|C): (\d+)")?;

    let mut computer = Computer::new();

    for (_, [reg, val]) in re.captures_iter(parts[0]).map(|c| c.extract()) {
        let int_val = val.parse::<i64>()?;
        match reg {
            "A" => computer.reg.a = int_val,
            "B" => computer.reg.b = int_val,
            "C" => computer.reg.c = int_val,
            _ => panic!(),
        }
    }

    let program: Vec<u8> = parts[1][9..]
        .split(",")
        .map(|i| i.trim().parse::<u8>().unwrap())
        .collect();

    cli::stage(1, || -> i64 {
        let res = computer.clone().run(&program);
        let output = res.iter().map(|r| r.to_string()).join(",");
        println!("{}", output);
        return 0;
    });

    cli::stage(2, || -> i64 {
        let result = recurse(&mut computer.clone(), &program, 0, 0, &program.clone()).unwrap();

        return result;
    });

    Ok(())
}

fn recurse(
    computer: &mut Computer,
    program: &Vec<u8>,
    fixed: i64,
    place: usize,
    expected: &Vec<u8>,
) -> Option<i64> {
    if place as usize > expected.len() - 1 {
        return None;
    }

    for o in 0_i64..=7 {
        let val = fixed + (o << ((expected.len() - 1 - place) * 3));
        let mut new_computer = computer.clone();
        new_computer.reg.a = val;
        let res = new_computer.run(program);
        if &res == expected {
            return Some(val);
        }

        // find last portion of answer
        let needle = res.len() as isize - (place as isize);
        // println!("needle: {}", needle);

        // skip if answer toot short
        if needle > res.len() as isize {
            continue;
        }

        if expected.ends_with(&res[needle as usize..]) {
            // search next digit
            let found = recurse(computer, program, val, place + 1, expected);
            if found.is_some() {
                return found;
            }
        }
    }

    return None;
}

#[derive(Debug, Clone)]
struct Computer {
    reg: Registry,
    pointer: usize,
    res: Vec<u8>,
}

#[derive(Debug, Clone)]
struct Registry {
    a: i64,
    b: i64,
    c: i64,
}

impl Registry {
    fn new() -> Self {
        Self { a: 0, b: 0, c: 0 }
    }
}

impl Computer {
    fn new() -> Self {
        Self {
            reg: Registry::new(),
            pointer: 0,
            res: Vec::new(),
        }
    }

    fn run(&mut self, program: &Vec<u8>) -> Vec<u8> {
        loop {
            if program.get(self.pointer).is_none() {
                break;
            }

            let instr = program[self.pointer];
            let operand = program[self.pointer + 1];

            self.step(instr, operand);
        }

        return self.res.clone();
    }

    fn step(&mut self, instr: u8, operand: u8) {
        match instr {
            0 => self.adv(operand),
            1 => self.bxl(operand),
            2 => self.bst(operand),
            3 => self.jnz(operand),
            4 => self.bxc(operand),
            5 => self.out(operand),
            6 => self.bdv(operand),
            7 => self.cdv(operand),
            _ => panic!(),
        }
    }

    fn combo(&self, operand: u8) -> i64 {
        match operand {
            0..=3 => operand as i64,
            4 => self.reg.a,
            5 => self.reg.b,
            6 => self.reg.c,
            7 => panic!(),
            _ => panic!(),
        }
    }

    fn adv(&mut self, operand: u8) {
        self.reg.a = self.reg.a / 2_i64.pow(self.combo(operand) as u32);
        self.pointer += 2;
    }

    fn bxl(&mut self, operand: u8) {
        self.reg.b = self.reg.b ^ operand as i64;
        self.pointer += 2;
    }

    fn bst(&mut self, operand: u8) {
        self.reg.b = self.combo(operand) % 8;
        self.pointer += 2;
    }

    fn jnz(&mut self, operand: u8) {
        if self.reg.a == 0 {
            self.pointer += 2;
            return;
        }
        self.pointer = operand as usize;
    }

    fn bxc(&mut self, _: u8) {
        self.reg.b = self.reg.b ^ self.reg.c;
        self.pointer += 2;
    }

    fn out(&mut self, operand: u8) {
        self.res.push((self.combo(operand) % 8) as u8);
        self.pointer += 2;
    }

    fn bdv(&mut self, operand: u8) {
        self.reg.b = self.reg.a / 2_i64.pow(self.combo(operand) as u32);
        self.pointer += 2;
    }

    fn cdv(&mut self, operand: u8) {
        self.reg.c = self.reg.a / 2_i64.pow(self.combo(operand) as u32);
        self.pointer += 2;
    }
}

impl fmt::Display for Computer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "A: {}, B: {}, C: {}, pointer: {}",
            self.reg.a, self.reg.b, self.reg.c, self.pointer
        )
    }
}
