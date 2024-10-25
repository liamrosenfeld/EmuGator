pub use std::collections::{HashMap, BTreeMap};

#[derive(Debug, Clone, PartialEq)]
pub enum Format {
    R, 
    I,
    S,
    B,
    U,
    J,
}

#[derive(Debug, Clone)]
pub struct InstructionInfo {
    format: Format,
    opcode: u32,
    funct3: Option<u32>,
    funct7: Option<u32>,
}

struct InstructionSet {
    instructions: HashMap<String, InstructionInfo>
}

#[derive(Debug)]
struct Instruction {
    info: InstructionInfo,
    rd: Option<u32>,
    rs1: Option<u32>,
    rs2: Option<u32>,
    immediate: Option<i32>
}

impl InstructionSet {
    fn new() -> Self {
        let mut instructions = HashMap::new();
        
        // R-type instructions
        instructions.insert("ADD".to_string(), InstructionInfo {
            format: Format::R,
            opcode: 0b0110011,
            funct3: Some(0x0),
            funct7: Some(0x00),
        });
        instructions.insert("SUB".to_string(), InstructionInfo {
            format: Format::R,
            opcode: 0b0110011,
            funct3: Some(0x0),
            funct7: Some(0x20),
        });
        instructions.insert("SLT".to_string(), InstructionInfo {
            format: Format::R,
            opcode: 0b0110011,
            funct3: Some(0x2),
            funct7: Some(0x00),
        });
        instructions.insert("SLTU".to_string(), InstructionInfo {
            format: Format::R,
            opcode: 0b0110011,
            funct3: Some(0x3),
            funct7: Some(0x00),
        });
        instructions.insert("AND".to_string(), InstructionInfo {
            format: Format::R,
            opcode: 0b0110011,
            funct3: Some(0x7),
            funct7: Some(0x00),
        });
        instructions.insert("OR".to_string(), InstructionInfo {
            format: Format::R,
            opcode: 0b0110011,
            funct3: Some(0x6),
            funct7: Some(0x00),
        });
        instructions.insert("XOR".to_string(), InstructionInfo {
            format: Format::R,
            opcode: 0b0110011,
            funct3: Some(0x4),
            funct7: Some(0x00),
        });
        instructions.insert("SLL".to_string(), InstructionInfo {
            format: Format::R,
            opcode: 0b0110011,
            funct3: Some(0x1),
            funct7: Some(0x00),
        });
        instructions.insert("SRL".to_string(), InstructionInfo {
            format: Format::R,
            opcode: 0b0110011,
            funct3: Some(0x5),
            funct7: Some(0x00),
        });
        instructions.insert("SRA".to_string(), InstructionInfo {
            format: Format::R,
            opcode: 0b0110011,
            funct3: Some(0x5),
            funct7: Some(0x20),
        });

        // I-type instructions
        instructions.insert("ADDI".to_string(), InstructionInfo {
            format: Format::I,
            opcode: 0b0010011,
            funct3: Some(0x0),
            funct7: None,
        });
        instructions.insert("SLTI".to_string(), InstructionInfo {
            format: Format::I,
            opcode: 0b0010011,
            funct3: Some(0x2),
            funct7: None,
        });
        instructions.insert("SLTIU".to_string(), InstructionInfo {
            format: Format::I,
            opcode: 0b0010011,
            funct3: Some(0x3),
            funct7: None,
        });
        instructions.insert("ANDI".to_string(), InstructionInfo {
            format: Format::I,
            opcode: 0b0010011,
            funct3: Some(0x7),
            funct7: None,
        });
        instructions.insert("ORI".to_string(), InstructionInfo {
            format: Format::I,
            opcode: 0b0010011,
            funct3: Some(0x6),
            funct7: None,
        });
        instructions.insert("XORI".to_string(), InstructionInfo {
            format: Format::I,
            opcode: 0b0010011,
            funct3: Some(0x4),
            funct7: None,
        });
        instructions.insert("SLLI".to_string(), InstructionInfo {
            format: Format::I,
            opcode: 0b0010011,
            funct3: Some(0x1),
            funct7: Some(0x00),
        });
        instructions.insert("SRLI".to_string(), InstructionInfo {
            format: Format::I,
            opcode: 0b0010011,
            funct3: Some(0x5),
            funct7: Some(0x00),
        });
        instructions.insert("SRAI".to_string(), InstructionInfo {
            format: Format::I,
            opcode: 0b0010011,
            funct3: Some(0x5),
            funct7: Some(0x20),
        });
        instructions.insert("JALR".to_string(), InstructionInfo {
            format: Format::I,
            opcode: 0b1100111,
            funct3: Some(0x0),
            funct7: None,
        });
        
        // Load instructions (I-type)
        instructions.insert("LW".to_string(), InstructionInfo {
            format: Format::I,
            opcode: 0b0000011,
            funct3: Some(0x2),
            funct7: None,
        });
        instructions.insert("LH".to_string(), InstructionInfo {
            format: Format::I,
            opcode: 0b0000011,
            funct3: Some(0x1),
            funct7: None,
        });
        instructions.insert("LHU".to_string(), InstructionInfo {
            format: Format::I,
            opcode: 0b0000011,
            funct3: Some(0x5),
            funct7: None,
        });
        instructions.insert("LB".to_string(), InstructionInfo {
            format: Format::I,
            opcode: 0b0000011,
            funct3: Some(0x0),
            funct7: None,
        });
        instructions.insert("LBU".to_string(), InstructionInfo {
            format: Format::I,
            opcode: 0b0000011,
            funct3: Some(0x4),
            funct7: None,
        });

        // Special I-type instructions
        instructions.insert("FENCE".to_string(), InstructionInfo {
            format: Format::I,
            opcode: 0b0001111,
            funct3: Some(0x0),
            funct7: None,
        });
        instructions.insert("ECALL".to_string(), InstructionInfo {
            format: Format::I,
            opcode: 0b1110011,
            funct3: Some(0x0),
            funct7: None,
        });
        instructions.insert("EBREAK".to_string(), InstructionInfo {
            format: Format::I,
            opcode: 0b1110011,
            funct3: Some(0x0),
            funct7: None,
        });

        // S-type instructions
        instructions.insert("SW".to_string(), InstructionInfo {
            format: Format::S,
            opcode: 0b0100011,
            funct3: Some(0x2),
            funct7: None,
        });
        instructions.insert("SH".to_string(), InstructionInfo {
            format: Format::S,
            opcode: 0b0100011,
            funct3: Some(0x1),
            funct7: None,
        });
        instructions.insert("SB".to_string(), InstructionInfo {
            format: Format::S,
            opcode: 0b0100011,
            funct3: Some(0x0),
            funct7: None,
        });

        // B-type instructions
        instructions.insert("BEQ".to_string(), InstructionInfo {
            format: Format::B,
            opcode: 0b1100011,
            funct3: Some(0x0),
            funct7: None,
        });
        instructions.insert("BNE".to_string(), InstructionInfo {
            format: Format::B,
            opcode: 0b1100011,
            funct3: Some(0x1),
            funct7: None,
        });
        instructions.insert("BLT".to_string(), InstructionInfo {
            format: Format::B,
            opcode: 0b1100011,
            funct3: Some(0x4),
            funct7: None,
        });
        instructions.insert("BLTU".to_string(), InstructionInfo {
            format: Format::B,
            opcode: 0b1100011,
            funct3: Some(0x6),
            funct7: None,
        });
        instructions.insert("BGE".to_string(), InstructionInfo {
            format: Format::B,
            opcode: 0b1100011,
            funct3: Some(0x5),
            funct7: None,
        });
        instructions.insert("BGEU".to_string(), InstructionInfo {
            format: Format::B,
            opcode: 0b1100011,
            funct3: Some(0x7),
            funct7: None,
        });

        // U-type instructions
        instructions.insert("LUI".to_string(), InstructionInfo {
            format: Format::U,
            opcode: 0b0110111,
            funct3: None,
            funct7: None,
        });
        instructions.insert("AUIPC".to_string(), InstructionInfo {
            format: Format::U,
            opcode: 0b0010111,
            funct3: None,
            funct7: None,
        });

        // J-type instructions
        instructions.insert("JAL".to_string(), InstructionInfo {
            format: Format::J,
            opcode: 0b1101111,
            funct3: None,
            funct7: None,
        });

        InstructionSet { instructions }
    }

    fn get(&self, name: &str) -> Option<InstructionInfo> {
        self.instructions.get(name).cloned()
    }
}

struct Assembler {
    instruction_set: InstructionSet,
}

impl Assembler {
    fn new() -> Self {
        Assembler {
            instruction_set: InstructionSet::new(),
        }
    }

    fn assemble(&self, program: &str) -> Result<AssembledProgram, String> {
        let mut assembled = AssembledProgram::new();
        let mut address = 0;

        // First pass: collect labels
        for (_line_num, line) in program.lines().enumerate() {
            let line = self.clean_line(line);
            if line.is_empty() { continue; }
            
            if line.ends_with(':') {
                let label = line.trim_end_matches(':').to_string();
                assembled.add_label(label, address);
                continue;
            }

            address += 4;
        }

        // Second pass: assemble instructions
        address = 0;
        for (line_num, line) in program.lines().enumerate() {
            let line = self.clean_line(line);
            if line.is_empty() || line.ends_with(':') { continue; }

            match self.parse_instruction(&line, &assembled.labels, address) {
                Ok(instruction) => {
                    let encoded = self.encode_instruction(&instruction);
                    assembled.add_instruction(address, encoded, line_num + 1);
                    address += 4;
                }
                Err(e) => return Err(format!("Error on line {}: {}", line_num + 1, e)),
            }
        }

        Ok(assembled)
    }

    fn clean_line(&self, line: &str) -> String {
        match line.split('#').next() {
            Some(l) => l.trim().to_string(),
            None => String::new(),
        }
    }

    fn parse_instruction(&self, line: &str, labels: &HashMap<String, u32>, current_address: u32) 
        -> Result<Instruction, String> {
        let parts: Vec<&str> = line.split(|c| c == ' ' || c == ',')
            .filter(|s| !s.is_empty())
            .collect();

        if parts.is_empty() {
            return Err("Empty instruction".to_string());
        }

        let name = parts[0].to_uppercase();
        let info = self.instruction_set.get(&name)
            .ok_or(format!("Unknown instruction: {}", name))?;

        match info.format {
            Format::R => self.parse_r_type(&parts, info),
            Format::I => self.parse_i_type(&parts, info),
            Format::S => self.parse_s_type(&parts, info),
            Format::B => self.parse_b_type(&parts, info, labels, current_address),
            Format::U => self.parse_u_type(&parts, info),
            Format::J => self.parse_j_type(&parts, info, labels, current_address),
        }
    }

    fn parse_r_type(&self, parts: &[&str], info: InstructionInfo) -> Result<Instruction, String> {
        if parts.len() != 4 {
            return Err("R-type instructions need 3 registers".to_string());
        }

        Ok(Instruction {
            info,
            rd: Some(self.parse_register(parts[1])?),
            rs1: Some(self.parse_register(parts[2])?),
            rs2: Some(self.parse_register(parts[3])?),
            immediate: None,
        })
    }

    fn parse_i_type(&self, parts: &[&str], info: InstructionInfo) -> Result<Instruction, String> {
        match info.opcode {
            0b0000011 => self.parse_load_type(&parts, info), // Load instructions
            _ => {
                if parts.len() != 4 {
                    return Err("I-type instructions need 2 registers and an immediate".to_string());
                }

                Ok(Instruction {
                    info,
                    rd: Some(self.parse_register(parts[1])?),
                    rs1: Some(self.parse_register(parts[2])?),
                    rs2: None,
                    immediate: Some(self.parse_immediate(parts[3])?),
                })
            }
        }
    }

    fn parse_load_type(&self, parts: &[&str], info: InstructionInfo) -> Result<Instruction, String> {
        if parts.len() != 3 {
            return Err("Load instructions need a register and a memory address".to_string());
        }

        let (offset, base) = self.parse_mem_address(parts[2])?;
        
        Ok(Instruction {
            info,
            rd: Some(self.parse_register(parts[1])?),
            rs1: Some(base),
            rs2: None,
            immediate: Some(offset),
        })
    }

    fn parse_s_type(&self, parts: &[&str], info: InstructionInfo) -> Result<Instruction, String> {
        if parts.len() != 3 {
            return Err("Store instructions need a register and a memory address".to_string());
        }

        let (offset, base) = self.parse_mem_address(parts[2])?;
        
        Ok(Instruction {
            info,
            rd: None,
            rs1: Some(base),
            rs2: Some(self.parse_register(parts[1])?),
            immediate: Some(offset),
        })
    }

    fn parse_b_type(&self, parts: &[&str], info: InstructionInfo, 
        labels: &HashMap<String, u32>, current_address: u32) -> Result<Instruction, String> {
        if parts.len() != 4 {
            return Err("B-type instructions need 2 registers and a label".to_string());
        }
    
        let target = labels.get(parts[3])
            .ok_or(format!("Undefined label: {}", parts[3]))?;
        
        // Calculate offset and ensure it's aligned
        let offset = (*target as i32) - (current_address as i32);
        if offset & 1 != 0 {
            return Err("Branch target must be aligned to 2 bytes".to_string());
        }
    
        Ok(Instruction {
            info,
            rd: None,
            rs1: Some(self.parse_register(parts[1])?),
            rs2: Some(self.parse_register(parts[2])?),
            immediate: Some(offset),
        })
    }

    fn parse_u_type(&self, parts: &[&str], info: InstructionInfo) -> Result<Instruction, String> {
        if parts.len() != 3 {
            return Err("U-type instructions need a register and an immediate".to_string());
        }

        Ok(Instruction {
            info,
            rd: Some(self.parse_register(parts[1])?),
            rs1: None,
            rs2: None,
            immediate: Some(self.parse_immediate(parts[2])?),
        })
    }

    fn parse_j_type(&self, parts: &[&str], info: InstructionInfo,
        labels: &HashMap<String, u32>, current_address: u32) -> Result<Instruction, String> {
        if parts.len() != 3 {
            return Err("J-type instructions need a register and a label/offset".to_string());
        }
    
        let offset = if let Ok(imm) = self.parse_immediate(parts[2]) {
            // Check alignment for immediate values too
            if imm & 1 != 0 {
                return Err("Jump target must be aligned to 2 bytes".to_string());
            }
            imm
        } else {
            let target = labels.get(parts[2])
                .ok_or(format!("Undefined label: {}", parts[2]))?;
            let offset = (*target as i32) - (current_address as i32);
            if offset & 1 != 0 {
                return Err("Jump target must be aligned to 2 bytes".to_string());
            }
            offset
        };
    
        Ok(Instruction {
            info,
            rd: Some(self.parse_register(parts[1])?),
            rs1: None,
            rs2: None,
            immediate: Some(offset),
        })
    }

    fn parse_mem_address(&self, addr: &str) -> Result<(i32, u32), String> {
        let parts: Vec<&str> = addr.split(|c| c == '(' || c == ')')
            .filter(|s| !s.is_empty())
            .collect();
        
        if parts.len() != 2 {
            return Err("Memory address must be in format: offset(register)".to_string());
        }

        let offset = self.parse_immediate(parts[0])?;
        let reg = self.parse_register(parts[1])?;
        
        Ok((offset, reg))
    }

    fn parse_register(&self, reg: &str) -> Result<u32, String> {
        let reg = reg.trim().to_lowercase();
        if !reg.starts_with('x') {
            return Err(format!("Invalid register: {}", reg));
        }
        
        match reg[1..].parse::<u32>() {
            Ok(num) if num < 32 => Ok(num),
            _ => Err(format!("Invalid register number: {}", reg)),
        }
    }

    fn parse_immediate(&self, imm: &str) -> Result<i32, String> {
        let imm = imm.trim();
        if imm.starts_with("0x") {
            i32::from_str_radix(&imm[2..], 16)
        } else {
            imm.parse::<i32>()
        }.map_err(|_| format!("Invalid immediate value: {}", imm))
    }

    fn encode_instruction(&self, inst: &Instruction) -> u32 {
        match inst.info.format {
            Format::R => {
                let rd = inst.rd.unwrap_or(0) & 0x1F;
                let rs1 = inst.rs1.unwrap_or(0) & 0x1F;
                let rs2 = inst.rs2.unwrap_or(0) & 0x1F;
                let funct3 = inst.info.funct3.unwrap_or(0) & 0x7;
                let funct7 = inst.info.funct7.unwrap_or(0) & 0x7F;
                
                (funct7 << 25) | (rs2 << 20) | (rs1 << 15) | 
                (funct3 << 12) | (rd << 7) | inst.info.opcode
            },
            Format::I => {
                let rd = inst.rd.unwrap_or(0) & 0x1F;
                let rs1 = inst.rs1.unwrap_or(0) & 0x1F;
                let imm = (inst.immediate.unwrap_or(0) & 0xFFF) as u32;
                let funct3 = inst.info.funct3.unwrap_or(0) & 0x7;
                
                (imm << 20) | (rs1 << 15) | (funct3 << 12) | 
                (rd << 7) | inst.info.opcode
            },
            Format::S => {
                let rs1 = inst.rs1.unwrap_or(0) & 0x1F;
                let rs2 = inst.rs2.unwrap_or(0) & 0x1F;
                let imm = inst.immediate.unwrap_or(0) as u32;
                let funct3 = inst.info.funct3.unwrap_or(0) & 0x7;
                
                let imm_11_5 = ((imm >> 5) & 0x7F) << 25;
                let imm_4_0 = (imm & 0x1F) << 7;
                
                imm_11_5 | (rs2 << 20) | (rs1 << 15) | 
                (funct3 << 12) | imm_4_0 | inst.info.opcode
            },
            Format::B => {
                let rs1 = inst.rs1.unwrap_or(0) & 0x1F;
                let rs2 = inst.rs2.unwrap_or(0) & 0x1F;
                let imm = inst.immediate.unwrap_or(0) as u32;
                let funct3 = inst.info.funct3.unwrap_or(0) & 0x7;
                
                let imm_12 = ((imm >> 12) & 0x1) << 31;
                let imm_11 = ((imm >> 11) & 0x1) << 7;
                let imm_10_5 = ((imm >> 5) & 0x3F) << 25;
                let imm_4_1 = ((imm >> 1) & 0xF) << 8;
                
                imm_12 | imm_10_5 | (rs2 << 20) | (rs1 << 15) |
                (funct3 << 12) | imm_4_1 | imm_11 | inst.info.opcode
            },
            Format::U => {
                let rd = inst.rd.unwrap_or(0) & 0x1F;
                let imm = (inst.immediate.unwrap_or(0) as u32) << 12;
                
                imm | (rd << 7) | inst.info.opcode
            }
            Format::J => {
                let rd = inst.rd.unwrap_or(0) & 0x1F;
                let imm = inst.immediate.unwrap_or(0) as u32;
                
                let imm_20 = ((imm >> 20) & 0x1) << 31;
                let imm_10_1 = ((imm >> 1) & 0x3FF) << 21;
                let imm_11 = ((imm >> 11) & 0x1) << 20;
                let imm_19_12 = ((imm >> 12) & 0xFF) << 12;
                
                imm_20 | imm_10_1 | imm_11 | imm_19_12 |
                (rd << 7) | inst.info.opcode
            }
        }
    }
}

#[derive(Debug)]
struct AssembledProgram {
    instruction_memory: BTreeMap<u32, u8>,
    source_map: BTreeMap<u32, usize>,
    labels: HashMap<String, u32>,
}

impl AssembledProgram {
    fn new() -> Self {
        AssembledProgram {
            instruction_memory: BTreeMap::new(),
            source_map: BTreeMap::new(),
            labels: HashMap::new(),
        }
    }

    fn add_label(&mut self, label: String, address: u32) {
        self.labels.insert(label, address);
    }

    fn add_instruction(&mut self, address: u32, encoded: u32, line_num: usize) {
        self.instruction_memory.insert(address, (encoded & 0xFF) as u8);
        self.instruction_memory.insert(address + 1, ((encoded >> 8) & 0xFF) as u8);
        self.instruction_memory.insert(address + 2, ((encoded >> 16) & 0xFF) as u8);
        self.instruction_memory.insert(address + 3, ((encoded >> 24) & 0xFF) as u8);
        
        self.source_map.insert(address, line_num);
    }
} 

pub fn get_emulator_maps(program: &str) -> Result<(BTreeMap<u32, u8>, BTreeMap<u32, usize>, BTreeMap<u32, u8>), String> {
    let assembler = Assembler::new();
    match assembler.assemble(program) {
        Ok(assembled) => {
            // Create empty data memory map - this would be populated if we had data directives
            let data_memory: BTreeMap<u32, u8> = BTreeMap::new();
            
            Ok((
                assembled.instruction_memory,
                assembled.source_map,
                data_memory
            ))
        },
        Err(e) => Err(e)
    }
}