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

#[derive(PartialEq, Default)]
pub struct AtRegAddr {
    reg: String,
    off_is_reg: bool,
    offset_reg: String,
    offset_num: isize,
}

#[derive(PartialEq)]
pub enum Operand {
    Immediate(isize),
    ReadRegAddr(AtRegAddr),
    Register(String),
}

#[derive(Default)]
pub struct Instruction {
    prefix: Option<String>,
    opcode: Option<String>, // This is an option for dumb but convenient reasons.
    opsize: Option<usize>,
    oper0:  Option<Operand>,
    oper1:  Option<Operand>,
}

fn is_instruction_or_prefix(s: &str) -> bool {
    for c in s.chars() {
        if !c.is_alphanumeric() {return false}
    }
    true
}

pub fn parse(instruction: &str, sizes: HashMap<&str, usize>) {
    println!("Instruction: `{}`", instruction);
    let mut tokens: Vec<_> = instruction.split(" ").collect();
    tokens.reverse();
    let mut instr: Instruction = Instruction::default();
    for mut token in tokens {
        let tok_len: usize = token.len();
        if token.as_bytes()[tok_len - 1] as char == ',' {
            token = &token[0..tok_len - 1]
        }
        let mut n = token.parse::<isize>();
        if n.is_ok() {
            println!("Immediate: {}", n.clone().unwrap());
            if instr.oper1 == None {
                instr.oper0 = Some(Operand::Immediate(n.unwrap()));
            } else {
                instr.oper1 = Some(Operand::Immediate(n.unwrap()));
            }
            continue
        } else if token.to_lowercase() == "ptr" {
            continue
        } else if REGS.contains(&token) {
            println!("Register: {}", token);
            if instr.oper1 == None {
                instr.oper0 = Some(Operand::Register(token.to_string()));
            } else {
                instr.oper1 = Some(Operand::Register(token.to_string()));
            }
            continue
        } else if let Some(sz) = sizes.get(token.to_lowercase().as_str()) {
            println!("Operand size is {} bytes ({})", sz, token);
            instr.opsize = Some(*sz);
            continue
        } else if is_instruction_or_prefix(token) {
            if instr.opcode == None {
                println!("Instruction: {}", token);
                instr.opcode = Some(token.to_string());
            } else {
                println!("Prefix: {}", token);
                instr.prefix = Some(token.to_string());
            }
            continue
        } else if token.as_bytes()[tok_len - 1] as char == ']' {
            let mut arg: AtRegAddr = AtRegAddr::default();
            let lbracket_idx = token.find("[").unwrap();
            arg.reg = (&token[lbracket_idx + 1..tok_len - 1]).to_string();
            let off = &token[..lbracket_idx];
            print!("Offset read - Address in register: {}", arg.reg);
            if REGS.contains(&off) {
                arg.off_is_reg = true;
                arg.offset_reg = off.to_string();
                println!(", offset in register: {}", arg.offset_reg);
            } else {
                arg.off_is_reg = false;
                n = off.parse::<isize>();
                if !n.is_ok() {
                    panic!("Memory access offset is neither a register nor an immediate numeric value.");
                }
                arg.offset_num = n.unwrap();
                println!(", offset: {}", arg.offset_num);
            }
            if instr.oper1 == None {
                instr.oper0 = Some(Operand::ReadRegAddr(arg));
            } else {
                instr.oper1 = Some(Operand::ReadRegAddr(arg));
            }
            continue
        }
        println!("Tok: {}", token);
    }
}
