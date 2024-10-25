use std::collections::{HashMap, BTreeMap};

#[derive(Debug, Clone, PartialEq)]
pub enum Format {
    R, 
    I,
}

#[derive(Debug, Clone)]
pub struct InstructionInfo {
    name: String,
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
        
        instructions.insert("ADD".to_string(), InstructionInfo {
            name: "ADD".to_string(),
            format: Format::R,
            opcode: 0b0110011,
            funct3: Some(0x0),
            funct7: Some(0x00),
        });
        
        instructions.insert("ADDI".to_string(), InstructionInfo {
            name: "ADDI".to_string(),
            format: Format::I,
            opcode: 0b0010011,
            funct3: Some(0x0),
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

        for (line_num, line) in program.lines().enumerate() {
            let line = self.clean_line(line);
            if line.is_empty() { continue; }
            
            if line.ends_with(':') {
                let label = line.trim_end_matches(':').to_string();
                assembled.add_label(label, address);
                continue;
            }

            match self.parse_instruction(&line) {
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

    fn parse_instruction(&self, line: &str) -> Result<Instruction, String> {
        let parts: Vec<&str> = line.split(|c| c == ' ' || c == ',')
            .filter(|s| !s.is_empty())
            .collect();

        let name = parts[0].to_uppercase();
        let info = self.instruction_set.get(&name)
            .ok_or(format!("Unknown instruction: {}", name))?;

        match info.format {
            Format::R => self.parse_r_type(&parts, info),
            Format::I => self.parse_i_type(&parts, info),
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

    fn print(&self) {
        println!("Assembly successful!\n");
        println!("Assembled Instructions:");
        
        let mut address = 0u32;
        while address < (self.instruction_memory.len() as u32 / 4) * 4 {
            if let Some(&line_num) = self.source_map.get(&address) {
                let encoded = 
                    (*self.instruction_memory.get(&(address + 3)).unwrap_or(&0) as u32) << 24 |
                    (*self.instruction_memory.get(&(address + 2)).unwrap_or(&0) as u32) << 16 |
                    (*self.instruction_memory.get(&(address + 1)).unwrap_or(&0) as u32) << 8 |
                    (*self.instruction_memory.get(&address).unwrap_or(&0) as u32);
                
                println!("0x{:08X} (line {:3}): 0x{:08X}", address, line_num, encoded);
                println!("    Bytes: {:02X} {:02X} {:02X} {:02X}", 
                    encoded & 0xFF,
                    (encoded >> 8) & 0xFF,
                    (encoded >> 16) & 0xFF,
                    (encoded >> 24) & 0xFF
                );
            }
            address += 4;
        }
    }
}

fn main() {
    let test_program = r#"
start:
    ADDI x1, x0, 10
    ADDI x2, x0, 20
    ADDI x3, x0, 5
    
loop:
    ADD x4, x1, x2
    ADD x5, x4, x3
    ADDI x1, x1, 1
    ADDI x2, x2, 2
    
    ADD x6, x4, x5
    ADDI x7, x6, 15

end:
    ADD x8, x7, x6
"#;

    let assembler = Assembler::new();
    match assembler.assemble(test_program) {
        Ok(program) => {
            program.print();
            
            println!("\nLabels:");
            for (label, address) in &program.labels {
                println!("{}: 0x{:08X}", label, address);
            }
        }
        Err(e) => println!("Error: {}", e),
    }
}