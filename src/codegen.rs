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

fn convert_push(buf: &mut String, instr: Instruction, reg_equ: &HashMap<&str, &str>, _instructions: &Vec<Instruction>) {
    match instr.oper0.unwrap() {
        Operand::Register(r) => buf.push_str(format!(
            "STP {}, [sp, #-8]!\nMOV r6, r7\nMOV r7, {}",
            reg_equ.get(r.as_str()).expect("unknown register to push"),
            reg_equ.get(r.as_str()).expect("unknown register to push")
        ).as_str()),
        _ => todo!("so far only registers can be pushed onto the stack"),
    };
}

fn convert_pop(buf: &mut String, instr: Instruction, reg_equ: &HashMap<&str, &str>, _instructions: &Vec<Instruction>) {
    match instr.oper0.unwrap() {
        Operand::Register(r) => buf.push_str(format!(
            "LDR {}, [sp], #8\n", reg_equ.get(r.as_str()).expect("unknown register to pop")
        ).as_str()),
        _ => todo!("so far only registers can be popped from the stack"),
    };
}

fn convert_lea(buf: &mut String, instr: Instruction, reg_equ: &HashMap<&str, &str>, _instructions: &Vec<Instruction>) {
    match (instr.oper0, instr.oper1) {
        (Some(Operand::Register(r)), Some(Operand::ReadRegAddr(m))) => {
            if let AddrOffset::Label(label) = m.off {
                buf.push_str(format!(
                    "ADR r29, {}\nADD {}, r29, {}\n",
                    label, reg_equ.get(r.as_str()).expect("unknown register to lea"), reg_equ.get(m.reg.as_str()).expect("unknown register to lea")
                ).as_str());
            } else {
                panic!("lea operand 1 must always be a memory offset with a label");
            }
        }, 
        _ => todo!("so far operand 0 of lea can only be a register and operand 1 must always be a memory offset"),
    }
}

fn convert_mov(buf: &mut String, instr: Instruction, reg_equ: &HashMap<&str, &str>, _instructions: &Vec<Instruction>) {
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

fn convert_jmp(buf: &mut String, instr: Instruction, _reg_equ: &HashMap<&str, &str>, _instructions: &Vec<Instruction>) {
    match instr.oper0.unwrap() {
        Operand::Label(l) => buf.push_str(format!(
            "B {}\n", l
        ).as_str()),
        _ => todo!("so far jmp only supports jumping to a label"),
    }
}

fn convert_call(buf: &mut String, instr: Instruction, _reg_equ: &HashMap<&str, &str>, _instructions: &Vec<Instruction>) {
    match instr.oper0.unwrap() {
        Operand::Label(l) => buf.push_str(format!(
            "BL {}\n", l
        ).as_str()),
        _ => todo!("so far call only supports jumping to a label"),
    }
}

fn convert_ret(buf: &mut String, _instr: Instruction, _reg_equ: &HashMap<&str, &str>, _instructions: &Vec<Instruction>) {
    buf.push_str("RET\n");
}

fn convert_txt(buf: &mut String, _instr: Instruction, _reg_equ: &HashMap<&str, &str>, _instructions: &Vec<Instruction>) {
    buf.push_str(".section .text\n");
}

fn convert_globl(buf: &mut String, instr: Instruction, _reg_equ: &HashMap<&str, &str>, _instructions: &Vec<Instruction>) {
    match instr.oper0.unwrap() {
        Operand::Label(l) => buf.push_str(format!(".global {}\n", l).as_str()),
        _ => panic!("Invalid token to define as global"),
    }
}

fn convert_str(buf: &mut String, instr: Instruction, _reg_equ: &HashMap<&str, &str>, _instructions: &Vec<Instruction>) {
    match instr.oper0.unwrap() {
        Operand::Label(l) => buf.push_str(format!(".asciz {}\n", l).as_str()),
        _ => panic!("Invalid token for string literal"),
    }
}

fn convert_instruction(buf: &mut String, instr: Instruction, reg_equ: &HashMap<&str, &str>, instructions: &Vec<Instruction>) {
    if instr.label != None {
        buf.push_str(format!("{}:\n", instr.label.clone().unwrap()).as_str());
        return
    }
    match instr.opcode.as_str() {
        "push"   =>  convert_push(buf, instr, reg_equ, instructions),
        "pop"    =>   convert_pop(buf, instr, reg_equ, instructions),
        "mov"    =>   convert_mov(buf, instr, reg_equ, instructions),
        "jmp"    =>   convert_jmp(buf, instr, reg_equ, instructions),
        "lea"    =>   convert_lea(buf, instr, reg_equ, instructions),
        "call"   =>  convert_call(buf, instr, reg_equ, instructions),
        "ret"    =>   convert_ret(buf, instr, reg_equ, instructions),
        ".text"  =>   convert_txt(buf, instr, reg_equ, instructions),
        ".globl" => convert_globl(buf, instr, reg_equ, instructions),
        ".str"   =>   convert_str(buf, instr, reg_equ, instructions),
        _ => todo!("Instruction not implemented yet: {:?}", instr),
    };
}

pub fn gen_arm64(parsed: Vec<Instruction>) {
    let mut buf = String::new();
    // NOTE: Don't map r6 or r7 to anything!
    let reg_equ = HashMap::from([
        ("rbp", "r29"),
        ("rsp", "sp" ),
        ("rip", "pc" ),
        ("rax", "r0" ),
        ("eax", "w0" ),
        ("rdi", "r0" ),
    ]);
    for instruction in &parsed {
        convert_instruction(&mut buf, instruction.clone(), &reg_equ, &parsed);
    }
    println!("Generated arm64 asm:\n\n{}", buf);
}
