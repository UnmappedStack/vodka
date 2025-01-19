use std::collections::HashMap;
mod lexer;

fn main() {
    let sizes = HashMap::from([
        ("byte" , 1),
        ("word" , 2),
        ("dword", 4),
        ("qword", 8),
    ]);
    let instr: &str = "lock mov rax, 3";
    lexer::lex(instr, sizes.clone());
    println!();
    let instr: &str = "mov rax, DWORD PTR -12[rbp]";
    lexer::lex(instr, sizes);
}
