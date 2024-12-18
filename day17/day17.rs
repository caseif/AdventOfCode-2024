use itertools::Itertools;
use num_enum::TryFromPrimitive;
use aoc2024_common::file::read_input_lines;

fn main() {
    let input = parse_input();
    println!("Part 1: {}", solve_p1(&input));
    println!("Part 2: {}", solve_p2(&input));
}

fn solve_p1(input: &ComputerInput) -> String {
    let mut computer = Computer::from_input(input);
    do_simulation(&mut computer);
    computer.out.iter().map(u8::to_string).join(",")
}

fn solve_p2(input: &ComputerInput) -> u64 {
    let mut computer = Computer::from_input(input);

    let target = computer.text.clone();

    let mut exp = computer.text.len() - 1;
    let mut i = 1u64 << (exp * 3);
    loop {
        computer.a = i;
        computer.b = 0;
        computer.c = 0;
        computer.ip = 0;
        computer.out.clear();

        do_simulation(&mut computer);

        //println!("{:o} -> {:?}", i, computer.out);

        if computer.out.len() <= target.len() && computer.out.len() > exp && computer.out[exp] == target[exp] {
            if exp == 0 {
                return i;
            } else {
                exp -= 1;
                continue;
            }
        }

        let mask = 7u64 << (exp * 3);
        if i & mask == mask {
            // prefix doesn't work, need to backtrack
            i &= !((1u64 << ((exp + 1) * 3)) - 1);
            exp += 1;
            if exp >= target.len() {
                panic!("Failed to find solution");
            }
        }
        i += 1u64 << (exp * 3);
    }
}

fn parse_input() -> ComputerInput {
    let lines = read_input_lines(17);
    ComputerInput {
        a: lines[0].split_once(":").unwrap().1.trim().parse::<u64>().unwrap(),
        b: lines[1].split_once(":").unwrap().1.trim().parse::<u64>().unwrap(),
        c: lines[2].split_once(":").unwrap().1.trim().parse::<u64>().unwrap(),
        text: lines[3].split_once(": ").unwrap().1.trim().split(",").map(|s| s.parse::<u8>().unwrap()).collect(),
    }
}

fn do_simulation(computer: &mut Computer) {
    let mut steps = 0;
    while let Some(_) = do_simulation_step(computer) {
        steps += 1;
        if steps > 100000 {
            panic!("Program does not appear to halt");
        }
    }
}

fn do_simulation_step(computer: &mut Computer) -> Option<()> {
    match computer.read_opcode()? {
        Opcode::Adv => {
            let operand = computer.read_combo_operand()?;
            computer.a /= 2u64.pow(operand as u32);
        }
        Opcode::Bxl => {
            let operand = computer.read_literal_operand()?;
            computer.b ^= operand as u64;
        }
        Opcode::Bst => {
            let operand = computer.read_combo_operand()?;
            computer.b = operand & 0x07
        }
        Opcode::Jnz => {
            let operand = computer.read_literal_operand()?;
            if computer.a != 0 {
                computer.ip = operand;
            }
        }
        Opcode::Bxc => {
            let _ = computer.read_combo_operand()?;
            computer.b ^= computer.c;
        }
        Opcode::Out => {
            let operand = computer.read_combo_operand()?;
            let out_val = (operand & 0x07) as u8;
            computer.out.push(out_val);
        }
        Opcode::Bdv => {
            let operand = computer.read_combo_operand()?;
            computer.b = computer.a / 2u64.pow(operand as u32);
        }
        Opcode::Cdv => {
            let operand = computer.read_combo_operand()?;
            computer.c = computer.a / 2u64.pow(operand as u32);
        }
    }
    Some(())
}

struct ComputerInput {
    a: u64,
    b: u64,
    c: u64,
    text: Vec<u8>,
}

struct Computer {
    a: u64,
    b: u64,
    c: u64,
    ip: u8,
    text: Vec<u8>,
    out: Vec<u8>,
}

impl Computer {
    fn from_input(input: &ComputerInput) -> Self {
        Self {
            a: input.a,
            b: input.b,
            c: input.c,
            ip: 0,
            text: input.text.clone(),
            out: Vec::new(),
        }
    }

    fn is_ip_valid(&self) -> Option<()> {
        if (self.ip as usize) < self.text.len() {
            Some(())
        } else {
            None
        }
    }

    fn read_byte(&mut self) -> Option<u8> {
        self.is_ip_valid()?;
        let val = self.text[self.ip as usize];
        self.ip += 1;
        Some(val)
    }

    fn read_opcode(&mut self) -> Option<Opcode> {
        Some(Opcode::try_from(self.read_byte()?).unwrap())
    }

    fn read_literal_operand(&mut self) -> Option<u8> {
        self.read_byte()
    }

    fn read_combo_operand(&mut self) -> Option<u64> {
        let val = self.read_byte()?;
        Some(match val {
            0..=3 => val as u64,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => panic!(),
        })
    }
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, TryFromPrimitive)]
enum Opcode {
    Adv = 0,
    Bxl = 1,
    Bst = 2,
    Jnz = 3,
    Bxc = 4,
    Out = 5,
    Bdv = 6,
    Cdv = 7,
}
