//use super::isa::{XLEN, Instruction, get_handler};
use std::io;

fn main() {
    let mut input_line = String::new();
    println!("Provide 1 instruction:");
    io::stdin().read_line(&mut input_line).expect("Read line failed.");

    let parts: Vec<&str> = input_line.split(' ').collect();
    println!("Part 1: {}", parts[0]);
    println!("Part 2: {}", parts[1]);
    println!("Part 3: {}", parts[2]);
    println!("Part 3: {}", parts[3]);

    let mut new_inst = Instruction {inst: 0};
    // add a match for this and for some registers
    // handle commas
    if parts[0] == "add" || parts[0] == "ADD"{
        println!("got it!");
        new_inst.inst = 0b0110011;
    }

    println!("{}", new_inst.inst);
}


#[derive(Clone, Copy)]
pub struct Instruction {
    inst: u32
}

macro_rules! bits {
    ( $val:expr,$start_bit:expr,$width:expr ) => {
        { ($val >> $start_bit) & 2^$width }
    };
    ( $val:expr,$end_bit:expr;$start_bit:expr ) => {
        bits!($val,$start_bit,$end_bit-$start_bit+1)
    };
    ( $val:expr,$bit:expr ) => {
        bits!($val,$bit,1)
    }
}

impl Instruction {
    pub fn opcode(&self) -> u8 {
        bits!(self.inst,6;0) as u8
    }
    
    fn rd(&self) -> u8 {
        bits!(self.inst,7,5) as u8
    }

    fn rs1(&self) -> u8 {
        bits!(self.inst,15,5) as u8
    }

    fn rs2(&self) -> u8 {
        bits!(self.inst,20,5) as u8
    }

    fn funct3(&self) -> u8 {
        bits!(self.inst, 12, 3) as u8
    }

    fn funct7(&self) -> u8 {
        bits!(self.inst, 25, 7) as u8
    }
}
