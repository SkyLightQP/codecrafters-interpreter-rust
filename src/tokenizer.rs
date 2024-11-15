use std::io::{self, Write};

pub fn tokenize(input: &str) -> i32 {
    let mut line = 1;
    let mut latest_error_code = 0;
    let mut chars = input.chars().peekable();

    while let Some(char) = chars.next() {
        match char {
            '(' => println!("LEFT_PAREN ( null"),
            ')' => println!("RIGHT_PAREN ) null"),
            '{' => println!("LEFT_BRACE {{ null"),
            '}' => println!("RIGHT_BRACE }} null"),
            '*' => println!("STAR * null"),
            '.' => println!("DOT . null"),
            ',' => println!("COMMA , null"),
            '+' => println!("PLUS + null"),
            '-' => println!("MINUS - null"),
            ';' => println!("SEMICOLON ; null"),
            '=' => {
                if let Some(&'=') = chars.peek() {
                    chars.next();
                    println!("EQUAL_EQUAL == null");
                } else {
                    println!("EQUAL = null");
                }
            }
            '!' => {
                if let Some(&'=') = chars.peek() {
                    chars.next();
                    println!("BANG_EQUAL != null");
                } else {
                    println!("BANG ! null");
                }
            }
            '>' => {
                if let Some(&'=') = chars.peek() {
                    chars.next();
                    println!("GREATER_EQUAL >= null");
                } else {
                    println!("GREATER > null");
                }
            }
            '<' => {
                if let Some(&'=') = chars.peek() {
                    chars.next();
                    println!("LESS_EQUAL <= null");
                } else {
                    println!("LESS < null");
                }
            }
            '\n' => line += 1,
            _ => {
                writeln!(
                    io::stderr(),
                    "[line {}] Error: Unexpected character: {}",
                    line,
                    char
                )
                .unwrap();
                latest_error_code = 65;
            }
        }
    }

    return latest_error_code;
}
