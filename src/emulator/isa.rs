use super::emulator::{EmulatorState, InstructionHandler};
use super::datapath::CVE2Pipeline;

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

// The underscores and allow dead code are used to suppress a huge flood of warnings.
// They should be removed as each function is implemented

#[allow(dead_code)]
fn LUI(_inst: Instruction, _state: EmulatorState) -> EmulatorState {
    todo!()
}

#[allow(dead_code)]
fn AUIPC(_inst: Instruction, _state: EmulatorState) -> EmulatorState {
    todo!()
}

#[allow(dead_code)]
fn JAL(_inst: Instruction, _state: EmulatorState) -> EmulatorState {
    todo!()
}

#[allow(dead_code)]
fn JALR(_inst: Instruction, _state: EmulatorState) -> EmulatorState {
    todo!()
}

#[allow(dead_code)]
fn BEQ(_inst: Instruction, _state: EmulatorState) -> EmulatorState {
    todo!()
}

#[allow(dead_code)]
fn BNE(_inst: Instruction, _state: EmulatorState) -> EmulatorState {
    todo!()
}
      
#[allow(dead_code)]        
fn BLT(_inst: Instruction, _state: EmulatorState) -> EmulatorState {
    todo!()
}

#[allow(dead_code)]
fn BGE(_inst: Instruction, _state: EmulatorState) -> EmulatorState {
    todo!()
}

#[allow(dead_code)]
fn BLTU(_inst: Instruction, _state: EmulatorState) -> EmulatorState {
    todo!()
}

#[allow(dead_code)]
fn BGEU(_inst: Instruction, _state: EmulatorState) -> EmulatorState {
    todo!()
}

#[allow(dead_code)]
fn LB(_inst: Instruction, _state: EmulatorState) -> EmulatorState {
    todo!()
}

#[allow(dead_code)]
fn LH(_inst: Instruction, _state: EmulatorState) -> EmulatorState {
    todo!()
}

#[allow(dead_code)]
fn LW(_inst: Instruction, _state: EmulatorState) -> EmulatorState {
    todo!()
}

#[allow(dead_code)]
fn LBU(_inst: Instruction, _state: EmulatorState) -> EmulatorState {
    todo!()
}

#[allow(dead_code)]
fn LHU(_inst: Instruction, _state: EmulatorState) -> EmulatorState {
    todo!()
}

#[allow(dead_code)]
fn SB(_inst: Instruction, _state: EmulatorState) -> EmulatorState {
    todo!()
}

#[allow(dead_code)]
fn SH(_inst: Instruction, _state: EmulatorState) -> EmulatorState {
    todo!()
}

#[allow(dead_code)]
fn SW(_inst: Instruction, _state: EmulatorState) -> EmulatorState {
    todo!()
}

#[allow(dead_code)]
fn ADDI(_inst: Instruction, _state: EmulatorState) -> EmulatorState {
    todo!()
}

#[allow(dead_code)]
fn SLTI(_inst: Instruction, _state: EmulatorState) -> EmulatorState {
    todo!()
}

#[allow(dead_code)]
fn SLTIU(_inst: Instruction, _state: EmulatorState) -> EmulatorState {
    todo!()
}

#[allow(dead_code)]
fn XORI(_inst: Instruction, _state: EmulatorState) -> EmulatorState {
    todo!()
}

#[allow(dead_code)]
fn ORI(_inst: Instruction, _state: EmulatorState) -> EmulatorState {
    todo!()
}

#[allow(dead_code)]
fn ANDI(_inst: Instruction, _state: EmulatorState) -> EmulatorState {
    todo!()
}

#[allow(dead_code)]
fn SLLI(_inst: Instruction, _state: EmulatorState) -> EmulatorState {
    todo!()
}

#[allow(dead_code)]
fn SRLI(_inst: Instruction, _state: EmulatorState) -> EmulatorState {
    todo!()
}

#[allow(dead_code)]
fn SRAI(_inst: Instruction, _state: EmulatorState) -> EmulatorState {
    todo!()
}

#[allow(dead_code)]
fn ADD(_inst: Instruction, _state: EmulatorState) -> EmulatorState {
    todo!()
}

#[allow(dead_code)]
fn SUB(_inst: Instruction, _state: EmulatorState) -> EmulatorState {
    todo!()
}

#[allow(dead_code)]
fn SLL(_inst: Instruction, _state: EmulatorState) -> EmulatorState {
    todo!()
}

#[allow(dead_code)]
fn SLT(_inst: Instruction, _state: EmulatorState) -> EmulatorState {
    todo!()
}

#[allow(dead_code)]
fn SLTU(_inst: Instruction, _state: EmulatorState) -> EmulatorState {
    todo!()
}

#[allow(dead_code)]
fn XOR(_inst: Instruction, _state: EmulatorState) -> EmulatorState {
    todo!()
}

#[allow(dead_code)]
fn SRL(_inst: Instruction, _state: EmulatorState) -> EmulatorState {
    todo!()
}

#[allow(dead_code)]
fn SRA(_inst: Instruction, _state: EmulatorState) -> EmulatorState {
    todo!()
}

#[allow(dead_code)]
fn OR(_inst: Instruction, _state: EmulatorState) -> EmulatorState {
    todo!()
}

#[allow(dead_code)]
fn AND(_inst: Instruction, _state: EmulatorState) -> EmulatorState {
    todo!()
}

#[allow(dead_code)]
fn FENCE(_inst: Instruction, _state: EmulatorState) -> EmulatorState {
    todo!()
}

#[allow(dead_code)]
fn FENCE_TSO(_inst: Instruction, _state: EmulatorState) -> EmulatorState {
    todo!()
}

#[allow(dead_code)]
fn PAUSE(_inst: Instruction, _state: EmulatorState) -> EmulatorState {
    todo!()
}

#[allow(dead_code)]
fn ECALL(_inst: Instruction, _state: EmulatorState) -> EmulatorState {
    todo!()
}

#[allow(dead_code)]
fn EBREAK(_inst: Instruction, _state: EmulatorState) -> EmulatorState {
    todo!()
}

#[allow(dead_code)]
fn CSRRW(_inst: Instruction, _state: EmulatorState) -> EmulatorState {
    todo!()
}

#[allow(dead_code)]
fn CSRRS(_inst: Instruction, _state: EmulatorState) -> EmulatorState {
    todo!()
}

#[allow(dead_code)]
fn CSRRC(_inst: Instruction, _state: EmulatorState) -> EmulatorState {
    todo!()
}

#[allow(dead_code)]
fn CSRRWI(_inst: Instruction, _state: EmulatorState) -> EmulatorState {
    todo!()
}

#[allow(dead_code)]
fn CSRRSI(_inst: Instruction, _state: EmulatorState) -> EmulatorState {
    todo!()
}

#[allow(dead_code)]
fn CSRRCI(_inst: Instruction, _state: EmulatorState) -> EmulatorState {
    todo!()
}
