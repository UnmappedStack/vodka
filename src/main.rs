mod lexer;

fn main() {
    let instr: &str = "lock mov rax, 3";
    lexer::lex(instr);
    println!();
    let instr: &str = "mov rax, 3[rbp]";
    lexer::lex(instr);
}
