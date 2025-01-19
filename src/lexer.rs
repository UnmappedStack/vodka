/* This is kinda bad since the lexer + parser are combined into one. For a compiler or interpreter,
 * I'd never do this, but for an assembler I think it actually simplifies things a little. */

#![allow(dead_code)]

const REGS: [&str; 69] = [
    "rax", "rbx", "rcx", "rdx",  "r8",  "r9",  "r10",  "r11",  "r12",  "r13",  "r14",  "r15", "rsi", "rdi", "rbp", "rsp", "rip",
    "eax", "ebx", "ecx", "edx", "r8d", "r9d", "r10d", "r11d", "r12d", "r13d", "r14d", "r15d", "esi", "edi", "ebp", "esp", "eip",
    "ax",  "bx",  "cx",  "dx", "r8w", "r9w", "r10w", "r11w", "r12w", "r13w", "r14w", "r15w",  "si",  "di",  "bp",  "sp",  "ip",
    "ah", "al", "bh", "bl", "ch", "cl", "dh", "dl", "r9b", "r10b", "r11b", "r12b", "r13b", "r14b", "r15b", "sil", "bpl", "spl"
];

#[derive(PartialEq)]
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

pub fn lex(instruction: &str) {
    println!("Instruction: `{}`", instruction);
    let mut tokens: Vec<_> = instruction.split(" ").collect();
    tokens.reverse();
    let mut instr: Instruction = Instruction::default();
    for mut token in tokens {
        if token.as_bytes()[token.len() - 1] as char == ',' {
            token = &token[0..token.len() - 1]
        }
        let n = token.parse::<isize>();
        if n.is_ok() {
            println!("Immediate: {}", n.clone().unwrap());
            if instr.oper1 == None {
                instr.oper0 = Some(Operand::Immediate(n.unwrap()));
            } else {
                instr.oper1 = Some(Operand::Immediate(n.unwrap()));
            }
            continue
        } else if REGS.contains(&token) {
            println!("Register: {}", token);
            if instr.oper1 == None {
                instr.oper0 = Some(Operand::Register(token.to_string()));
            } else {
                instr.oper1 = Some(Operand::Register(token.to_string()));
            }
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
        } else if token.as_bytes()[token.len() - 1] as char == ']' {
            todo!("Memory read argument types");
        }
        println!("Tok: {}", token);
    }
}
