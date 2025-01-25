#![allow(dead_code)]
use crate::parser::*;
use std::collections::HashMap;

fn check_internal_fn(instructions: &Vec<Instruction>, label: &str) -> bool {
    for instr in instructions {
        match &instr.label {
            Some(l) => if l == label {return true},
            _ => {},
        };
    }
    return false;
}

fn convert_push(buf: &mut String, instr: Instruction, reg_equ: &HashMap<&str, &str>, _instructions: &Vec<Instruction>, _line: usize) {
    match instr.oper0.unwrap() {
        Operand::Register(r) => buf.push_str(format!(
            "SUB SP, SP, #16\nSTR {}, [SP]\nMOV x6, x7\nMOV x7, {}\n",
            reg_equ.get(r.as_str()).expect("unknown register to push"),
            reg_equ.get(r.as_str()).expect("unknown register to push")
        ).as_str()),
        _ => todo!("so far only registers can be pushed onto the stack"),
    };
}

fn convert_pop(buf: &mut String, instr: Instruction, reg_equ: &HashMap<&str, &str>, _instructions: &Vec<Instruction>, _line: usize) {
    match instr.oper0.unwrap() {
        Operand::Register(r) => buf.push_str(format!(
            "LDR {}, [SP]\nADD SP, SP, #16\n", reg_equ.get(r.as_str()).expect("unknown register to pop")
        ).as_str()),
        _ => todo!("so far only registers can be popped from the stack"),
    };
}

fn convert_lea(buf: &mut String, instr: Instruction, reg_equ: &HashMap<&str, &str>, _instructions: &Vec<Instruction>, _line: usize) {
    match (instr.oper0, instr.oper1) {
        (Some(Operand::Register(r)), Some(Operand::ReadRegAddr(m))) => {
            if let AddrOffset::Label(label) = m.off {
                let reg2 = *reg_equ.get(m.reg.as_str()).expect("unknown register to lea");
                if reg2 == "." {
                    buf.push_str(format!(
                        "ADR {}, {}\n",
                        reg_equ.get(r.as_str()).expect("unknown register to lea"), label 
                    ).as_str());
                    return
                }
                buf.push_str(format!(
                    "ADR x29, {}\nADD {}, x29, {}\n",
                    label, reg_equ.get(r.as_str()).expect("unknown register to lea"), reg2
                ).as_str());
            } else {
                panic!("lea operand 1 must always be a memory offset with a label");
            }
        }, 
        _ => todo!("so far operand 0 of lea can only be a register and operand 1 must always be a memory offset"),
    }
}

fn convert_mov(buf: &mut String, instr: Instruction, reg_equ: &HashMap<&str, &str>, _instructions: &Vec<Instruction>, _line: usize) {
    match (instr.oper0, instr.oper1) {
        (Some(Operand::Register(r0)), Some(Operand::Register(r1))) => {
            buf.push_str(format!(
                "MOV {}, {}\n", 
                reg_equ.get(r0.as_str()).expect("unknown register to mov"),
                reg_equ.get(r1.as_str()).expect("unknown register to mov")
            ).as_str())
        },
        (Some(Operand::Register(r0)), Some(Operand::Immediate(n1))) => {
            buf.push_str(format!(
                "MOV {}, #{}\n",
                reg_equ.get(r0.as_str()).expect("unknown register to mov"),
                n1
            ).as_str());
        },
        _ => todo!("mov format not yet implemented"),
    };
}

fn convert_jmp(buf: &mut String, instr: Instruction, _reg_equ: &HashMap<&str, &str>, _instructions: &Vec<Instruction>, _line: usize) {
    match instr.oper0.unwrap() {
        Operand::Label(l) => buf.push_str(format!(
            "B {}\n", l
        ).as_str()),
        _ => todo!("so far jmp only supports jumping to a label"),
    }
}

fn convert_call(buf: &mut String, instr: Instruction, _reg_equ: &HashMap<&str, &str>, instructions: &Vec<Instruction>, _line: usize) {
    match instr.oper0.unwrap() {
        Operand::Label(l) => {
            let is_external = !check_internal_fn(instructions, &l);
            buf.push_str(format!(
                "BL {}\n", l
            ).as_str());
            if is_external {
                buf.push_str("MOV x28, x0\n");
            }
        },
        _ => todo!("so far call only supports jumping to a label"),
    }
}

fn convert_ret(buf: &mut String, _instr: Instruction, _reg_equ: &HashMap<&str, &str>, _instructions: &Vec<Instruction>, _line: usize) {
    buf.push_str("LDR x30, [SP]\nADD SP, SP, #16\nMOV x0, x28\nRET\n");
}

fn convert_txt(buf: &mut String, _instr: Instruction, _reg_equ: &HashMap<&str, &str>, _instructions: &Vec<Instruction>, _line: usize) {
    buf.push_str(".section .text\n");
}

fn convert_startfn(buf: &mut String, _instr: Instruction, _reg_equ: &HashMap<&str, &str>, _instructions: &Vec<Instruction>, _line: usize) {
    buf.push_str("SUB SP, SP, #16\nSTR x30, [SP]\n");
}

fn convert_section(buf: &mut String, instr: Instruction, _reg_equ: &HashMap<&str, &str>, _instructions: &Vec<Instruction>, _line: usize) {
    let sect_name = match instr.oper0 {
        Some(Operand::Label(l)) => l,
        _ => panic!(".section must have operand 0 as type label."),
    };
    buf.push_str(format!(".section {}\n", sect_name).as_str());
}

fn convert_globl(buf: &mut String, instr: Instruction, _reg_equ: &HashMap<&str, &str>, _instructions: &Vec<Instruction>, _line: usize) {
    match instr.oper0.unwrap() {
        Operand::Label(l) => buf.push_str(format!(".global {}\n", l).as_str()),
        _ => panic!("Invalid token to define as global"),
    }
}

fn convert_str(buf: &mut String, instr: Instruction, _reg_equ: &HashMap<&str, &str>, _instructions: &Vec<Instruction>, _line: usize) {
    match instr.oper0.unwrap() {
        Operand::Label(l) => buf.push_str(format!(".asciz {}\n", l).as_str()),
        _ => panic!("Invalid token for string literal"),
    }
}

fn convert_instruction(buf: &mut String, instr: Instruction, reg_equ: &HashMap<&str, &str>, instructions: &Vec<Instruction>, line: usize) {
    if instr.label != None {
        buf.push_str(format!("{}:\n", instr.label.clone().unwrap()).as_str());
        return
    }
    match instr.opcode.as_str() {
        "push"           =>    convert_push(buf, instr, reg_equ, instructions, line),
        "pop"            =>     convert_pop(buf, instr, reg_equ, instructions, line),
        "mov"            =>     convert_mov(buf, instr, reg_equ, instructions, line),
        "jmp"            =>     convert_jmp(buf, instr, reg_equ, instructions, line),
        "lea"            =>     convert_lea(buf, instr, reg_equ, instructions, line),
        "call"           =>    convert_call(buf, instr, reg_equ, instructions, line),
        "ret"            =>     convert_ret(buf, instr, reg_equ, instructions, line),
        ".text"          =>     convert_txt(buf, instr, reg_equ, instructions, line),
        ".globl"         =>   convert_globl(buf, instr, reg_equ, instructions, line),
        ".str"           =>     convert_str(buf, instr, reg_equ, instructions, line),
        ".section"       => convert_section(buf, instr, reg_equ, instructions, line),
        ".cfi_startproc" => convert_startfn(buf, instr, reg_equ, instructions, line),
        _ => todo!("Instruction not implemented yet: {:?}", instr),
    };
}

pub fn gen_arm64(parsed: Vec<Instruction>) -> String {
    let mut buf = String::new();
    // NOTE: Don't map r6 or r7 to anything!
    let reg_equ = HashMap::from([
        ("rbp", "x29"),
        ("rsp", "SP" ),
        ("rip", "."  ),
        ("rax", "x0" ),
        ("eax", "w0" ),
        ("rdi", "x28"),
    ]);
    for (i, instruction) in (&parsed).into_iter().enumerate() {
        convert_instruction(&mut buf, instruction.clone(), &reg_equ, &parsed, i);
    }
    println!("Generated arm64 asm:\n\n{}", buf);
    buf
}
