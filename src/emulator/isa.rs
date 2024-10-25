use super::emulator::{EmulatorState, InstructionHandler};
use super::datapath::CVE2Datapath;

pub type XLEN = i32;

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

pub fn get_handler(inst: Instruction) -> Result<InstructionHandler, ()> {
    match (inst.opcode(), inst.funct3(), inst.funct7()) {
        (0b0110111,     _,         _) => Ok(LUI),
        (0b0010111,     _,         _) => Ok(AUIPC),
        (0b1101111,     _,         _) => Ok(JAL),
        (0b1100111,     _,         _) => Ok(JALR),
        (0b1100011, 0b000,         _) => Ok(BEQ),
        (0b1100011, 0b001,         _) => Ok(BNE),
        (0b1100011, 0b100,         _) => Ok(BLT),
        (0b1100011, 0b101,         _) => Ok(BGE),
        (0b1100011, 0b110,         _) => Ok(BLTU),
        (0b1100011, 0b111,         _) => Ok(BGEU),
        (0b0000011, 0b000,         _) => Ok(LB),
        (0b0000011, 0b001,         _) => Ok(LH),
        (0b0000011, 0b010,         _) => Ok(LW),
        (0b0000011, 0b100,         _) => Ok(LBU),
        (0b0000011, 0b101,         _) => Ok(LHU),
        (0b0100011, 0b000,         _) => Ok(SB),
        (0b0100011, 0b001,         _) => Ok(SH),
        (0b0100011, 0b010,         _) => Ok(SW),
        (0b0010011, 0b000,         _) => Ok(ADDI),
        (0b0010011, 0b010,         _) => Ok(SLTI),
        (0b0010011, 0b011,         _) => Ok(SLTIU),
        (0b0010011, 0b100,         _) => Ok(XORI),
        (0b0010011, 0b110,         _) => Ok(ORI),
        (0b0010011, 0b111,         _) => Ok(ANDI),
        (0b0010011, 0b001, 0b0000000) => Ok(SLLI),
        (0b0010011, 0b101, 0b0000000) => Ok(SRLI),
        (0b0010011, 0b101, 0b0100000) => Ok(SRAI),
        (0b0110011, 0b000, 0b0000000) => Ok(ADD),
        (0b0110011, 0b000, 0b0100000) => Ok(SUB),
        (0b0110011, 0b001, 0b0000000) => Ok(SLL),
        (0b0110011, 0b010, 0b0000000) => Ok(SLT),
        (0b0110011, 0b011, 0b0000000) => Ok(SLTU),
        (0b0110011, 0b100, 0b0000000) => Ok(XOR),
        (0b0110011, 0b101, 0b0000000) => Ok(SRL),
        (0b0110011, 0b101, 0b0100000) => Ok(SRA),
        (0b0110011, 0b110, 0b0000000) => Ok(OR),
        (0b0110011, 0b111, 0b0000000) => Ok(AND),
        (0b0001111, 0b000,         _) => match inst.inst {
            0b1000_0011_0011_00000_000_00000_0001111 => Ok(FENCE_TSO),
            0b0000_0001_0000_00000_000_00000_0001111 => Ok(PAUSE),
            _ => Ok(FENCE)
        },
        (0b1110011, 0b000, 0b0000000) => match inst.inst {
            0b0000_0000_0000_00000_000_00000_1110011 => Ok(ECALL),
            0b0000_0000_0001_00000_000_00000_1110011 => Ok(EBREAK),
            _ => Err(())
        },
        (0b1110011, 0b001,         _) => Ok(CSRRW),
        (0b1110011, 0b010,         _) => Ok(CSRRS),
        (0b1110011, 0b011,         _) => Ok(CSRRC),
        (0b1110011, 0b101,         _) => Ok(CSRRWI),
        (0b1110011, 0b110,         _) => Ok(CSRRSI),
        (0b1110011, 0b111,         _) => Ok(CSRRCI),
        _ => Err(())
    }
}

pub enum InstructionFormat {
    R, I, S, B, U, J, CUSTOM
}

#[derive(Clone, Copy)]
pub struct Instruction {
    inst: u32
}

impl Instruction {
    pub fn opcode(&self) -> u8 {
        bits!(self.inst,6;0) as u8
    }

    fn immediate(&self, format: InstructionFormat) -> Result<i32, ()> {
        match format {
            InstructionFormat::I => Ok((
                bits!(self.inst,31) * (2^21 << 11) +
                bits!(self.inst,30;25) << 5  + 
                bits!(self.inst,24;21) << 1  +
                bits!(self.inst,20   )
            ) as i32),
            InstructionFormat::S => Ok((
                bits!(self.inst,31) * (2^21 << 11) +
                bits!(self.inst,30;25) << 5  +
                bits!(self.inst,11;8 ) << 1  +
                bits!(self.inst,7    )
            ) as i32),
            InstructionFormat::B => Ok((
                bits!(self.inst,31) * (2^20 << 12) +
                bits!(self.inst,7    ) << 11 + 
                bits!(self.inst,30;25) << 5  +
                bits!(self.inst,11;8 ) << 1
            ) as i32),
            InstructionFormat::U => Ok((
                bits!(self.inst,31) * (2^1 << 31) +
                bits!(self.inst,30;20) << 20 +
                bits!(self.inst,19;12) << 12
            ) as i32),
            InstructionFormat::J => Ok((
                bits!(self.inst,31) * (2^12 << 20) +
                bits!(self.inst,19;12) << 12 +
                bits!(self.inst,20   ) << 11 +
                bits!(self.inst,30;25) << 5  +
                bits!(self.inst,24;21) << 1
            ) as i32),
            _ => Err(()) 
        }
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

fn LUI(inst: Instruction, state: EmulatorState) -> EmulatorState {
    todo!()
}

fn AUIPC(inst: Instruction, state: EmulatorState) -> EmulatorState {
    todo!()
}

fn JAL(inst: Instruction, state: EmulatorState) -> EmulatorState {
    todo!()
}

fn JALR(inst: Instruction, state: EmulatorState) -> EmulatorState {
    todo!()
}

fn BEQ(inst: Instruction, state: EmulatorState) -> EmulatorState {
    todo!()
}

fn BNE(inst: Instruction, state: EmulatorState) -> EmulatorState {
    todo!()
}
              
fn BLT(inst: Instruction, state: EmulatorState) -> EmulatorState {
    todo!()
}

fn BGE(inst: Instruction, state: EmulatorState) -> EmulatorState {
    todo!()
}

fn BLTU(inst: Instruction, state: EmulatorState) -> EmulatorState {
    todo!()
}

fn BGEU(inst: Instruction, state: EmulatorState) -> EmulatorState {
    todo!()
}

fn LB(inst: Instruction, state: EmulatorState) -> EmulatorState {
    todo!()
}

fn LH(inst: Instruction, state: EmulatorState) -> EmulatorState {
    todo!()
}

fn LW(inst: Instruction, state: EmulatorState) -> EmulatorState {
    todo!()
}

fn LBU(inst: Instruction, state: EmulatorState) -> EmulatorState {
    todo!()
}

fn LHU(inst: Instruction, state: EmulatorState) -> EmulatorState {
    todo!()
}

fn SB(inst: Instruction, state: EmulatorState) -> EmulatorState {
    todo!()
}

fn SH(inst: Instruction, state: EmulatorState) -> EmulatorState {
    todo!()
}

fn SW(inst: Instruction, state: EmulatorState) -> EmulatorState {
    todo!()
}

fn ADDI(inst: Instruction, state: EmulatorState) -> EmulatorState {
    todo!()
}

fn SLTI(inst: Instruction, state: EmulatorState) -> EmulatorState {
    todo!()
}

fn SLTIU(inst: Instruction, state: EmulatorState) -> EmulatorState {
    todo!()
}

fn XORI(inst: Instruction, state: EmulatorState) -> EmulatorState {
    todo!()
}

fn ORI(inst: Instruction, state: EmulatorState) -> EmulatorState {
    todo!()
}

fn ANDI(inst: Instruction, state: EmulatorState) -> EmulatorState {
    todo!()
}

fn SLLI(inst: Instruction, state: EmulatorState) -> EmulatorState {
    todo!()
}

fn SRLI(inst: Instruction, state: EmulatorState) -> EmulatorState {
    todo!()
}

fn SRAI(inst: Instruction, state: EmulatorState) -> EmulatorState {
    todo!()
}

fn ADD(inst: Instruction, state: EmulatorState) -> EmulatorState {
    todo!()
}

fn SUB(inst: Instruction, state: EmulatorState) -> EmulatorState {
    todo!()
}

fn SLL(inst: Instruction, state: EmulatorState) -> EmulatorState {
    todo!()
}

fn SLT(inst: Instruction, state: EmulatorState) -> EmulatorState {
    todo!()
}

fn SLTU(inst: Instruction, state: EmulatorState) -> EmulatorState {
    todo!()
}

fn XOR(inst: Instruction, state: EmulatorState) -> EmulatorState {
    todo!()
}

fn SRL(inst: Instruction, state: EmulatorState) -> EmulatorState {
    todo!()
}

fn SRA(inst: Instruction, state: EmulatorState) -> EmulatorState {
    todo!()
}

fn OR(inst: Instruction, state: EmulatorState) -> EmulatorState {
    todo!()
}

fn AND(inst: Instruction, state: EmulatorState) -> EmulatorState {
    todo!()
}

fn FENCE(inst: Instruction, state: EmulatorState) -> EmulatorState {
    todo!()
}

fn FENCE_TSO(inst: Instruction, state: EmulatorState) -> EmulatorState {
    todo!()
}

fn PAUSE(inst: Instruction, state: EmulatorState) -> EmulatorState {
    todo!()
}

fn ECALL(inst: Instruction, state: EmulatorState) -> EmulatorState {
    todo!()
}

fn EBREAK(inst: Instruction, state: EmulatorState) -> EmulatorState {
    todo!()
}

fn CSRRW(inst: Instruction, state: EmulatorState) -> EmulatorState {
    todo!()
}

fn CSRRS(inst: Instruction, state: EmulatorState) -> EmulatorState {
    todo!()
}

fn CSRRC(inst: Instruction, state: EmulatorState) -> EmulatorState {
    todo!()
}

fn CSRRWI(inst: Instruction, state: EmulatorState) -> EmulatorState {
    todo!()
}

fn CSRRSI(inst: Instruction, state: EmulatorState) -> EmulatorState {
    todo!()
}

fn CSRRCI(inst: Instruction, state: EmulatorState) -> EmulatorState {
    todo!()
}
