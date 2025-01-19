use std::collections::HashMap;
mod parser;

fn main() {
    let sizes = HashMap::from([
        ("byte" , 1),
        ("word" , 2),
        ("dword", 4),
        ("qword", 8),
    ]);
    let instr: &str = "lock mov rax, 3";
    parser::parse(instr, sizes.clone());
    println!();
    let instr: &str = "mov rax, DWORD PTR -12[rbp]";
    parser::parse(instr, sizes);
}
