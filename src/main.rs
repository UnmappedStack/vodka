use std::collections::HashMap;
use std::fs;
mod parser;
mod codegen;

fn main() {
    let sizes = HashMap::from([
        ("byte" , 1),
        ("word" , 2),
        ("dword", 4),
        ("qword", 8),
    ]);
    let x86asm = fs::read_to_string("test.S")
                    .expect("Couldn't find test.S x86_64 assembly file for testing.");
    let parsed = parser::parse_file(x86asm, sizes);
    codegen::gen_arm64(parsed);
}
