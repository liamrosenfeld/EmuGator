use super::{EmulatorState, InstructionHandler, bits};

pub type XLEN = u32;

pub fn get_handler(instr: Instruction) -> Result<InstructionHandler, ()> {
    match (instr.opcode(), instr.funct3(), instr.funct7()) {
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
        (0b0001111, 0b000,         _) => match instr.instr {
            0b1000_0011_0011_00000_000_00000_0001111 => Ok(FENCE_TSO),
            0b0000_0001_0000_00000_000_00000_0001111 => Ok(PAUSE),
            _ => Ok(FENCE)
        },
        (0b1110011, 0b000, 0b0000000) => match instr.instr {
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
    pub(crate) instr: u32
}

impl Instruction {
    pub fn opcode(&self) -> u8 {
        bits!(self.instr,6;0) as u8
    }

    fn immediate(&self, format: InstructionFormat) -> Result<i32, ()> {
        match format {
            InstructionFormat::I => Ok((
                bits!(self.instr,31   ) * ((2^21 - 1) << 11) +
                bits!(self.instr,30;25) << 5  + 
                bits!(self.instr,24;21) << 1  +
                bits!(self.instr,20   )
            ) as i32),
            InstructionFormat::S => Ok((
                bits!(self.instr,31   ) * ((2^21 - 1) << 11) +
                bits!(self.instr,30;25) << 5  +
                bits!(self.instr,11;8 ) << 1  +
                bits!(self.instr,7    )
            ) as i32),
            InstructionFormat::B => Ok((
                bits!(self.instr,31   ) * ((2^20 - 1) << 12) +
                bits!(self.instr,7    ) << 11 + 
                bits!(self.instr,30;25) << 5  +
                bits!(self.instr,11;8 ) << 1
            ) as i32),
            InstructionFormat::U => Ok((
                bits!(self.instr,31   ) * ((2^1 - 1) << 31) +
                bits!(self.instr,30;20) << 20 +
                bits!(self.instr,19;12) << 12
            ) as i32),
            InstructionFormat::J => Ok((
                bits!(self.instr,31   ) * ((2^12 - 1) << 20) +
                bits!(self.instr,19;12) << 12 +
                bits!(self.instr,20   ) << 11 +
                bits!(self.instr,30;25) << 5  +
                bits!(self.instr,24;21) << 1
            ) as i32),
            _ => Err(()) 
        }
    }
    
    fn rd(&self) -> u8 {
        bits!(self.instr,7,5) as u8
    }

    fn rs1(&self) -> u8 {
        bits!(self.instr,15,5) as u8
    }

    fn rs2(&self) -> u8 {
        bits!(self.instr,20,5) as u8
    }

    fn funct3(&self) -> u8 {
        bits!(self.instr, 12, 3) as u8
    }

    fn funct7(&self) -> u8 {
        bits!(self.instr, 25, 7) as u8
    }
}

fn LUI(instr: &Instruction, state: &mut EmulatorState) {
    todo!()
}

fn AUIPC(instr: &Instruction, state: &mut EmulatorState) {
    todo!()
}

fn JAL(instr: &Instruction, state: &mut EmulatorState) {
    // TODO: Push onto Return Address stack when rd = x1/x5
    let immed = (instr.immediate(InstructionFormat::J)).unwrap();
    let new_pc = state.pc.checked_add_signed(immed).unwrap();
    
    // if unaligned on 4-byte boundary
    if(new_pc & 0x00000003 != 0x00){
        panic!("JAL instruction immediate it not on a 4-byte boundary");
    }
    // stores pc+4 into rd
    let rd = instr.rd() as usize;
    state.x[rd] = state.pc + 4;

    // update PC
    state.pc = new_pc;
}

fn JALR(instr: &Instruction, state: &mut EmulatorState) {
    // TODO: Push onto RAS
    let immed = (instr.immediate(InstructionFormat::I)).unwrap();
    let new_pc = state.pc.checked_add_signed(immed).unwrap() + state.x[instr.rs1() as usize] & 0xFFFFFFFE;

    // if unaligned on 4-byte boundary
    if(new_pc & 0x003 != 0x00){
        panic!("JAL target addess is not on a 4-byte boundary");
    }

    // stores pc+4 into rd
    let rd = instr.rd() as usize;
    state.x[rd] = state.pc + 4;

    // update PC
    state.pc = new_pc;
}

fn BEQ(instr: &Instruction, state: &mut EmulatorState) {
    let immed = (instr.immediate(InstructionFormat::B)).unwrap();
    let new_pc = state.pc.checked_add_signed(immed).unwrap();
    
    // if unaligned on 4-byte boundary
    if(new_pc & 0x003 != 0x00){
        panic!("JAL instruction immediate it not on a 4-byte boundary");
    }

    if(state.x[instr.rs1() as usize] == state.x[instr.rs2() as usize]){
        // update PC
        state.pc = new_pc;
    }
}

fn BNE(instr: &Instruction, state: &mut EmulatorState) {
    let immed = (instr.immediate(InstructionFormat::B)).unwrap();
    let new_pc = state.pc.checked_add_signed(immed).unwrap();
    
    // if unaligned on 4-byte boundary
    if(new_pc & 0x003 != 0x00){
        panic!("JAL instruction immediate it not on a 4-byte boundary");
    }

    if(state.x[instr.rs1() as usize] != state.x[instr.rs2() as usize]){
        // update PC
        state.pc = new_pc;
    }
}

fn BLT(instr: &Instruction, state: &mut EmulatorState) {
    let immed = (instr.immediate(InstructionFormat::B)).unwrap();
    let new_pc = state.pc.checked_add_signed(immed).unwrap();
    
    // if unaligned on 4-byte boundary
    if(new_pc & 0x003 != 0x00){
        panic!("JAL instruction immediate it not on a 4-byte boundary");
    }

    if((state.x[instr.rs1() as usize] as i8) < state.x[instr.rs2() as usize] as i8){
        // update PC
        state.pc = new_pc;
    }
}

fn BGE(instr: &Instruction, state: &mut EmulatorState) {
    let immed = (instr.immediate(InstructionFormat::B)).unwrap();
    let new_pc = state.pc.checked_add_signed(immed).unwrap();
    
    // if unaligned on 4-byte boundary
    if(new_pc & 0x003 != 0x00){
        panic!("JAL instruction immediate it not on a 4-byte boundary");
    }

    if((state.x[instr.rs1() as usize] as i8) > state.x[instr.rs2() as usize] as i8){
        // update PC
        state.pc = new_pc;
    }
}

fn BLTU(instr: &Instruction, state: &mut EmulatorState) {
    let immed = (instr.immediate(InstructionFormat::B)).unwrap();
    let new_pc = state.pc.checked_add_signed(immed).unwrap();
    
    // if unaligned on 4-byte boundary
    if(new_pc & 0x003 != 0x00){
        panic!("JAL instruction immediate it not on a 4-byte boundary");
    }

    if(state.x[instr.rs1() as usize] < state.x[instr.rs2() as usize]){
        // stores pc+4 into rd
        let rd = instr.rd() as usize;
        state.x[rd] = state.pc + 4;

        // update PC
        state.pc = new_pc;
    }
}

fn BGEU(instr: &Instruction, state: &mut EmulatorState) {
    let immed = (instr.immediate(InstructionFormat::B)).unwrap();
    let new_pc = state.pc.checked_add_signed(immed).unwrap();
    
    // if unaligned on 4-byte boundary
    if(new_pc & 0x003 != 0x00){
        panic!("JAL instruction immediate it not on a 4-byte boundary");
    }

    if(state.x[instr.rs1() as usize] > state.x[instr.rs2() as usize]){
        // stores pc+4 into rd
        let rd = instr.rd() as usize;
        state.x[rd] = state.pc + 4;

        // update PC
        state.pc = new_pc;
    }
}

fn LB(instr: &Instruction, state: &mut EmulatorState) {
    todo!()
}

fn LH(instr: &Instruction, state: &mut EmulatorState) {
    todo!()
}

fn LW(instr: &Instruction, state: &mut EmulatorState) {
    todo!()
}

fn LBU(instr: &Instruction, state: &mut EmulatorState) {
    todo!()
}

fn LHU(instr: &Instruction, state: &mut EmulatorState) {
    todo!()
}

fn SB(instr: &Instruction, state: &mut EmulatorState) {
    let data = state.x[instr.rs2() as usize] & 0xFF;
    let addr = state.x[instr.rs1() as usize] as i32 + instr.immediate(InstructionFormat::S).unwrap();

    // set data on pipline
    state.pipeline.datapath.data_addr_o = addr as u32;
    state.pipeline.datapath.data_we_o = true;
    state.pipeline.datapath.data_be_o = 0x1; // access LSB only
    state.pipeline.datapath.data_wdata_o = data;
}

fn SH(instr: &Instruction, state: &mut EmulatorState) {
    let data = state.x[instr.rs2() as usize];
    let addr = state.x[instr.rs1() as usize] as i32 + instr.immediate(InstructionFormat::S).unwrap();

    // set data on pipline
    state.pipeline.datapath.data_addr_o = addr as u32;
    state.pipeline.datapath.data_we_o = true;
    state.pipeline.datapath.data_be_o = 0x7; // access all 4 bytes
    state.pipeline.datapath.data_wdata_o = data;
}

fn SW(instr: &Instruction, state: &mut EmulatorState) {
    let data = state.x[instr.rs2() as usize] & 0xFFFF;
    let addr = state.x[instr.rs1() as usize] as i32 + instr.immediate(InstructionFormat::S).unwrap();

    // set data on pipline
    state.pipeline.datapath.data_addr_o = addr as u32;
    state.pipeline.datapath.data_we_o = true;
    state.pipeline.datapath.data_be_o = 0x3; // access last two bytes
    state.pipeline.datapath.data_wdata_o = data;
}

fn ADDI(instr: &Instruction, state: &mut EmulatorState) {
    let rd = instr.rd() as usize;
    let rs = instr.rs1() as usize;
    let immediate = instr.immediate(InstructionFormat::I).unwrap() as i32;

    // must match sign
    let rs = state.x[rs] as i32;

    state.x[rd] = (rs + immediate) as u32;
}

fn SLTI(instr: &Instruction, state: &mut EmulatorState) {
    let rd = instr.rd() as usize;
    let rs1 = instr.rs1() as usize;
    let immediate = instr.immediate(InstructionFormat::I).unwrap() as i32;

    // must treat as signed
    let rs1 = state.x[rs1] as i32;

    state.x[rd] = (rs1 < immediate) as u32;
}

fn SLTIU(instr: &Instruction, state: &mut EmulatorState) {
    let rd = instr.rd() as usize;
    let rs1 = instr.rs1() as usize;
    let immediate = instr.immediate(InstructionFormat::I).unwrap() as u32;

    state.x[rd] = (state.x[rs1] < immediate) as u32;
}

fn XORI(instr: &Instruction, state: &mut EmulatorState) {
    let rd = instr.rd() as usize;
    let rs = instr.rs1() as usize;
    let immediate = instr.immediate(InstructionFormat::I).unwrap() as u32;

    state.x[rd] = state.x[rs] ^ immediate;
}

fn ORI(instr: &Instruction, state: &mut EmulatorState) {
    let rd = instr.rd() as usize;
    let rs = instr.rs1() as usize;
    let immediate = instr.immediate(InstructionFormat::I).unwrap() as u32;

    state.x[rd] = state.x[rs] | immediate;
}

fn ANDI(instr: &Instruction, state: &mut EmulatorState) {
    let rd = instr.rd() as usize;
    let rs = instr.rs1() as usize;
    let immediate = instr.immediate(InstructionFormat::I).unwrap() as u32;

    state.x[rd] = state.x[rs] & immediate;
}

fn SLLI(instr: &Instruction, state: &mut EmulatorState) {
    let rd = instr.rd() as usize;
    let rs = instr.rs1() as usize;
    let immediate = instr.immediate(InstructionFormat::I).unwrap() as u32;

    // TODO: ask christo if I can ignore the 0x1F
    state.x[rd] = state.x[rs] << (immediate & 0x1F);
}

fn SRLI(instr: &Instruction, state: &mut EmulatorState) {
    let rd = instr.rd() as usize;
    let rs = instr.rs1() as usize;
    let immediate = instr.immediate(InstructionFormat::I).unwrap() as u32;

    // TODO: ask christo if I can ignore the 0x1F
    state.x[rd] = state.x[rs] << (immediate & 0x1F);
}

fn SRAI(instr: &Instruction, state: &mut EmulatorState) {
    let rd = instr.rd() as usize;
    let rs = instr.rs1() as usize;
    let immediate = instr.immediate(InstructionFormat::I).unwrap() as u32;

    // TODO: ask christo if I can ignore the 0x1F
    state.x[rd] = state.x[rs] >> (immediate & 0x1F);
}

fn ADD(instr: &Instruction, state: &mut EmulatorState) {
    let rd = instr.rd() as usize;
    let rs1 = instr.rs1() as usize;
    let rs2 = instr.rs2() as usize;

    state.x[rd] = state.x[rs1] + state.x[rs2];
}

fn SUB(instr: &Instruction, state: &mut EmulatorState) {
    let rd = instr.rd() as usize;
    let rs1 = instr.rs1() as usize;
    let rs2 = instr.rs2() as usize;

    state.x[rd] = state.x[rs1] - state.x[rs2];
}

fn SLL(instr: &Instruction, state: &mut EmulatorState) {
    let rd = instr.rd() as usize;
    let rs1 = instr.rs1() as usize;
    let rs2 = instr.rs2() as usize;

    state.x[rd] = state.x[rs1] << state.x[rs2];
}

fn SLT(instr: &Instruction, state: &mut EmulatorState) {
    let rd = instr.rd() as usize;
    let rs1 = instr.rs1() as usize;
    let rs2 = instr.rs2() as usize;

    // must treat both as signed
    let rs1 = state.x[rs1] as i32;
    let rs2 = state.x[rs2] as i32;

    state.x[rd] = (rs1 < rs2) as u32;
}

fn SLTU(instr: &Instruction, state: &mut EmulatorState) {
    let rd = instr.rd() as usize;
    let rs1 = instr.rs1() as usize;
    let rs2 = instr.rs2() as usize;

    state.x[rd] = (state.x[rs1] < state.x[rs2]) as u32;
}

fn XOR(instr: &Instruction, state: &mut EmulatorState) {
    let rd = instr.rd() as usize;
    let rs1 = instr.rs1() as usize;
    let rs2 = instr.rs2() as usize;

    state.x[rd] = state.x[rs1] ^ state.x[rs2];
}

fn SRL(instr: &Instruction, state: &mut EmulatorState) {
    let rd = instr.rd() as usize;
    let rs1 = instr.rs1() as usize;
    let rs2 = instr.rs2() as usize;

    // TODO: ask christo if I can ignore the 0x1F
    state.x[rd] = state.x[rs1] >> (state.x[rs2] & 0x1F);
}

fn SRA(instr: &Instruction, state: &mut EmulatorState) {
    let rd = instr.rd() as usize;
    let rs1 = instr.rs1() as usize;
    let rs2 = instr.rs2() as usize;

    state.x[rd] = state.x[rs1] >> state.x[rs2];
}

fn OR(instr: &Instruction, state: &mut EmulatorState) {
    let rd = instr.rd() as usize;
    let rs1 = instr.rs1() as usize;
    let rs2 = instr.rs2() as usize;

    state.x[rd] = state.x[rs1] | state.x[rs2];
}

fn AND(instr: &Instruction, state: &mut EmulatorState) {
    let rd = instr.rd() as usize;
    let rs1 = instr.rs1() as usize;
    let rs2 = instr.rs2() as usize;

    state.x[rd] = state.x[rs1] | state.x[rs2];
}

fn FENCE(instr: &Instruction, state: &mut EmulatorState) {
    todo!()
}

fn FENCE_TSO(instr: &Instruction, state: &mut EmulatorState) {
    todo!()
}

fn PAUSE(instr: &Instruction, state: &mut EmulatorState) {
    todo!()
}

fn ECALL(instr: &Instruction, state: &mut EmulatorState) {
    todo!()
}

fn EBREAK(instr: &Instruction, state: &mut EmulatorState) {
    todo!()
}

fn CSRRW(instr: &Instruction, state: &mut EmulatorState) {
    todo!()
}

fn CSRRS(instr: &Instruction, state: &mut EmulatorState) {
    todo!()
}

fn CSRRC(instr: &Instruction, state: &mut EmulatorState) {
    todo!()
}

fn CSRRWI(instr: &Instruction, state: &mut EmulatorState) {
    todo!()
}

fn CSRRSI(instr: &Instruction, state: &mut EmulatorState) {
    todo!()
}

fn CSRRCI(instr: &Instruction, state: &mut EmulatorState) {
    todo!()
}
