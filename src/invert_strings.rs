use std::io::{stdin, stdout, Write};

pub fn invert_string(input: &str) -> String {
    input.chars().rev().collect()
}


pub fn run(){
    let mut input = String::new();
    print!("Enter string: ");
    let _=stdout().flush();
    stdin().read_line(&mut input).expect("No se ingreso correctamente la cadena");
    if let Some('\n')=input.chars().next_back(){
        input.pop();
    }
    if let Some('\r')=input.chars().next_back(){
        input.pop();
    }
    println!("You typed: {}", input);
}


