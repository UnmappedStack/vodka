/* This is kinda bad since the lexer + parser are combined into one. For a compiler or interpreter,
 * I'd never do this, but for an assembler I think it actually simplifies things a little. */

#![allow(dead_code)]

use std::collections::HashMap;

const REGS: [&str; 69] = [
    "rax", "rbx", "rcx", "rdx",  "r8",  "r9",  "r10",  "r11",  "r12",  "r13",  "r14",  "r15", "rsi", "rdi", "rbp", "rsp", "rip",
    "eax", "ebx", "ecx", "edx", "r8d", "r9d", "r10d", "r11d", "r12d", "r13d", "r14d", "r15d", "esi", "edi", "ebp", "esp", "eip",
    "ax",  "bx",  "cx",  "dx", "r8w", "r9w", "r10w", "r11w", "r12w", "r13w", "r14w", "r15w",  "si",  "di",  "bp",  "sp",  "ip",
    "ah", "al", "bh", "bl", "ch", "cl", "dh", "dl", "r9b", "r10b", "r11b", "r12b", "r13b", "r14b", "r15b", "sil", "bpl", "spl"
];

const PREFIXES: [&str; 7] = [
    "lock", "repne", "repnz", "rep", "repe", "repz", "bnd"
];

#[derive(PartialEq, Debug, Clone)]
pub enum AddrOffset {
    Register(String),
    Number(isize),
    Label(String),
    Dflt // default
}

impl Default for AddrOffset {
    fn default() -> Self {
        AddrOffset::Dflt
    }
}

#[derive(PartialEq, Default, Debug, Clone)]
pub struct AtRegAddr {
    pub reg: String,
    pub off: AddrOffset,
}

#[derive(PartialEq, Debug, Clone)]
pub enum Operand {
    Immediate(isize),
    ReadRegAddr(AtRegAddr),
    Register(String),
    Label(String),
}

#[derive(Default, Debug, Clone)]
pub struct Instruction {
    pub label:  Option<String>,
    pub prefix: Option<String>,
    pub opcode: String,
    pub opsize: Option<usize>,
    pub oper0:  Option<Operand>,
    pub oper1:  Option<Operand>,
}

fn str_is_instruction(s: &str) -> bool {
    if s == ".text" || s == ".globl" {return true}
    for c in s.chars() {
        if !c.is_alphanumeric() {return false}
    }
    true
}

fn normalise_spaces(input: &str) -> String {
    let mut result = input.to_string();
    while result.contains("  ") || result.contains("\t") {
        result = result.replace("  ", " ");
        result = result.replace("\t", " ");
    }
    result
}

pub fn parse(instruction: &str, sizes: HashMap<&str, usize>) -> Option<Instruction> {
    if instruction.len() == 0 {return None}
    let mut instr: Instruction = Instruction::default();
    if instruction.chars().last() == Some(':') {
        instr.label = Some((&instruction[..instruction.len() - 1]).to_string());
        return Some(instr);
    }
    let mut tokens: Vec<_> = instruction.split(" ").collect();
    if tokens[0] == ".string" {
        instr.opcode = String::from(".str");
        instr.oper0 = Some(Operand::Label(tokens[1..].join(" ")));
        return Some(instr);
    }
    if instruction.as_bytes()[0] as char == '.' && !str_is_instruction(tokens[0]) {
        return None;
    }
    tokens.reverse();
    let mut tokens_iter = tokens.clone().into_iter().enumerate().peekable();
    while let Some((i, mut token)) = tokens_iter.next() {
        let mut tok_len: usize = token.len();
        let next_is_prefix = match tokens_iter.peek() {
            Some(t) => PREFIXES.contains(&t.1) && i == tokens.len() - 2,
            None => false,
        };
        if token.as_bytes()[tok_len - 1] as char == ',' {
            token = &token[0..tok_len - 1];
            tok_len -= 1;
        }
        let mut n = token.parse::<isize>();
        if n.is_ok() {
            if instr.oper1 == None {
                instr.oper1 = Some(Operand::Immediate(n.unwrap()));
            } else {
                instr.oper0 = Some(Operand::Immediate(n.unwrap()));
            }
            continue
        } else if token.to_lowercase() == "ptr" {
            continue
        } else if REGS.contains(&token) {
            if instr.oper1 == None {
                instr.oper1 = Some(Operand::Register(token.to_string()));
            } else {
                instr.oper0 = Some(Operand::Register(token.to_string()));
            }
            continue
        } else if let Some(sz) = sizes.get(token.to_lowercase().as_str()) {
            instr.opsize = Some(*sz);
            continue
        } else if PREFIXES.contains(&token) && i == tokens.len() - 1 {
            instr.prefix = Some(token.to_string());
            continue
        } else if str_is_instruction(token) &&
                  (next_is_prefix || i == tokens.len() - 1) {
            instr.opcode = token.to_string();
            continue
        } else if token.as_bytes()[tok_len - 1] as char == ']' {
            let mut arg: AtRegAddr = AtRegAddr::default();
            let lbracket_idx = token.find("[").unwrap();
            arg.reg = (&token[lbracket_idx + 1..tok_len - 1]).to_string();
            let off = &token[..lbracket_idx];
            if REGS.contains(&off) {
                arg.off = AddrOffset::Register(off.to_string());
            } else if (off.as_bytes()[0] as char).is_alphanumeric() || off.as_bytes()[0] as char == '.' {
                arg.off = AddrOffset::Label(off.to_string());
            } else {
                n = off.parse::<isize>();
                if !n.is_ok() {
                    panic!("Memory access offset is neither a register nor an immediate numeric value.");
                }
                arg.off = AddrOffset::Number(n.unwrap());
            }
            if instr.oper1 == None {
                instr.oper1 = Some(Operand::ReadRegAddr(arg));
            } else {
                instr.oper0 = Some(Operand::ReadRegAddr(arg));
            }
            continue
        } else if (token.as_bytes()[0] as char).is_alphanumeric() || token.as_bytes()[0] as char == '.' {
            if instr.oper1 == None {
                instr.oper1 = Some(Operand::Label(token.to_string()));
            } else {
                instr.oper0 = Some(Operand::Label(token.to_string()));
            }
        } else {
            panic!("Couldn't detect token type: {}", token);
        }
    }
    if instr.oper1 != None && instr.oper0 == None {
        instr.oper0 = instr.oper1;
        instr.oper1 = None;
    }
    return Some(instr);
}

pub fn parse_file(asm: String, sizes: HashMap<&str, usize>) -> Vec<Instruction> {
    let lines: Vec<_> = asm.split("\n").collect();
    let mut ret = Vec::new();
    for instruction in lines {
        let instr = parse(&normalise_spaces(instruction.trim_start()), sizes.clone());
        match instr {
            Some(i) => ret.push(i),
            _ => {},
        };
    }
    ret
}
