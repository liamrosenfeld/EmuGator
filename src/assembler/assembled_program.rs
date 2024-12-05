use bimap::BiBTreeMap;
use strum::EnumString;
use std::{
    char,
    collections::{BTreeMap, HashMap},
    fmt::{self, Formatter, UpperHex},
    ops::Add,
    str::FromStr,
    thread::current,
};
use web_sys::CharacterData;

use crate::isa::InstructionDefinition;

#[derive(Debug)]
pub struct AssemblyErr {
    pub line_num: usize,
    pub message: String,
}

#[derive(Debug)]
pub struct AssembledProgram {
    /// Map of instruction memory addresses to instruction bytes
    pub instruction_memory: BTreeMap<u32, u8>,

    /// Map of data memory addresses to data bytes
    pub data_memory: BTreeMap<u32, u8>,

    /// Map of line numbers (left) to addresses (right)
    pub source_map: BiBTreeMap<usize, Address>,

    /// Map of labels to addresses
    pub labels: HashMap<String, Address>,
}

impl AssembledProgram {
    pub fn new() -> Self {
        AssembledProgram {
            instruction_memory: BTreeMap::new(),
            data_memory: BTreeMap::new(),
            source_map: BiBTreeMap::new(),
            labels: HashMap::new(),
        }
    }

    fn write(&mut self, line_num: usize, address: Address, data: &[u8]) -> Result<(), AssemblyErr> {
        self.source_map.insert(line_num, address);

        let (addr, memory_map) = match address {
            Address::Text(addr) => (addr, &mut self.instruction_memory),
            Address::Data(addr) => (addr, &mut self.data_memory),
        };

        for (i, &byte) in data.iter().enumerate() {
            let addr = addr + i as u32;
            if memory_map.contains_key(&addr) {
                return Err(AssemblyErr {
                    line_num,
                    message: format!("Memory address {:X} is already in use", address),
                });
            }
            memory_map.insert(addr, byte);
        }

        Ok(())
    }
}

fn push_token(&mut tokens: Vec<Token>, token_start: usize, token_end: usize, line_num: usize, line: &str, address: Address) {
    if token_start < token_end {
        let token_text = &line[token_start..token_end];
        let token_kind = if token_text.starts_with('.') {
            TokenKind::Directive()
        } else if token_text.ends_with(':') {
            TokenKind::Label(token_text, address)
        } else {
            TokenKind::Instruction(InstructionDefinition::from(token_text))
        }
    }
}

impl FromStr for AssembledProgram {
    type Err = AssemblyErr;

    fn from_str(program: &str) -> Result<Self, Self::Err> {
        let mut data_memory: BTreeMap<u32, u8> = BTreeMap::new();
        let mut instruction_memory: BTreeMap<u32, u8> = BTreeMap::new();
        let mut source_map: BiBTreeMap<usize, Address> = BiBTreeMap::new();
        let mut labels: HashMap<String, Address> = HashMap::new();

        let mut current_address = Address::Text(0u32);

        // Tokenize the input
        let tokens: Vec<Token> = program
            .lines()
            .enumerate()
            .flat_map(|(line_num, line)| {
                let mut tokens: Vec<Token> = Vec::new();
                let mut token_start: usize = 0;
                let mut chars = line.chars().enumerate();

                while let Some((column, character)) = chars.next() {
                    // Whitespace and commas are token separators
                    if character.is_whitespace() || character == ',' {
                        push_token(tokens, token_start, column, line_num, line, current_address);
                        token_start = column + 1;
                        continue;
                    }

                    match character {
                        // Arithmetic operators are tokens and separators
                        '+' | '-' | '*' | '/' | '(' | ')' => {
                            if token_start < column {
                                push_token(tokens, token_start, column, line_num, line, current_address);
                            }
                            tokens.push(Token {
                                line_num,
                                column: pos,
                                kind: TokenKind::from(&line[pos..pos + 1]),
                            });
                            token_start = pos + 1;
                        }
                        // String literals are one token
                        '"' => {
                            let mut end = pos + 1;
                            let mut last_character = '"';
                            while let Some((pos, character)) = chars.next() {
                                // Unescaped quote
                                if character == '"' && last_character != '\\' {
                                    end = pos;
                                    break;
                                }

                                last_character = character;
                                end = pos;
                            }
                            tokens.push(Token {
                                line_num,
                                column: token_start,
                                kind: &line[token_start..end + 1].into(),
                            });
                            token_start = end + 1;
                        }
                        _ => {}
                    }
                }

                if token_start < line.len() {
                    tokens.push(Token {
                        line_num,
                        column: token_start,
                        kind: TokenKind::from(&line[token_start..]),
                    });
                }

                tokens
            })
            .collect();

        return Ok(AssembledProgram {
            data_memory,
            instruction_memory,
            source_map,
            labels,
        });

        Err(Self::Err {
            0,
            message: "Could not parse program".to_string(),
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Address {
    Text(u32),
    Data(u32),
}

impl UpperHex for Address {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Address::Text(addr) => write!(f, "Text({:#08X})", addr),
            Address::Data(addr) => write!(f, "Data({:#08X})", addr),
        }
    }
}
struct Token<'a> {
    pub line_num: usize,
    pub column: usize,
    pub kind: TokenKind<'a>,
    pub text: &'a str
}

enum TokenKind<'a> {
    Label(&'a str, Address),
    Instruction(&'static InstructionDefinition),
    Directive(Directive),
    Register(u8),
    Expression(Expression<'a>),
    String,
    Comment,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumString)]
#[strum(serialize_all = "UPPERCASE")]
enum Directive {
    Data,
    Text,
    Ascii,
    Asciiz,
    Byte,
    Half,
    Word,
    Space,
}

pub struct Expression<'a> {
    expression: &'a str,
    value: Option<Vec<i64>>,
}

impl<'a> From<&'a str> for Expression<'a> {
    fn from(s: &'a str) -> Expression<'a> {
        Expression {
            expression: s,
            value: None,
        }
    }
}

impl<'a> Expression<'a> {
    pub fn resolve(&mut self, labels: &HashMap<&str, Expression>) -> Result<&Vec<i64>, &str> {
        let value = vec![];

        let mut tokens: Vec<&str> = Vec::new();
        let mut current_token_start = 0;
        let mut chars = self.expression.chars().enumerate();

        while let Some((pos, character)) = chars.next() {
            // Whitespace and commas are token separators
            if character.is_whitespace() || character == ',' {
                if current_token_start < pos {
                    tokens.push(&self.expression[current_token_start..pos]);
                }
                current_token_start = pos + 1;
                continue;
            }

            match character {
                // Arithmetic operators are tokens and separators
                '+' | '-' | '*' | '/' | '(' | ')' => {
                    if current_token_start < pos {
                        tokens.push(&self.expression[current_token_start..pos]);
                    }
                    tokens.push(&self.expression[pos..pos + 1]);
                    current_token_start = pos + 1;
                }
                // String literals are one token
                '"' => {
                    let mut end = pos + 1;
                    let mut last_character = '"';
                    while let Some((pos, character)) = chars.next() {
                        // Unescaped quote
                        if character == '"' && last_character != '\\' {
                            end = pos;
                            break;
                        }

                        last_character = character;
                        end = pos;
                    }
                    tokens.push(&self.expression[pos..end + 1]);
                    current_token_start = end + 1;
                }
                _ => {}
            }
        }

        if current_token_start < self.expression.len() {
            tokens.push(&self.expression[current_token_start..]);
        }

        self.value = Some(value);
        match self.value {
            Some(ref v) => Ok(v),
            None => Err("Could not resolve literal"),
        }
    }
}
