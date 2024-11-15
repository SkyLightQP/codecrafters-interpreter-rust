use std::env;
use std::fs;
use std::io::{self, Write};
use std::process::exit;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        writeln!(io::stderr(), "Usage: {} tokenize <filename>", args[0]).unwrap();
        return;
    }

    let command = &args[1];
    let filename = &args[2];

    match command.as_str() {
        "tokenize" => {
            let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                writeln!(io::stderr(), "Failed to read file {}", filename).unwrap();
                String::new()
            });

            let result = tokenize(&file_contents);

            println!("EOF  null");

            if result != 0 {
                exit(result);
            }
        }
        _ => {
            writeln!(io::stderr(), "Unknown command: {}", command).unwrap();
            return;
        }
    }
}

fn tokenize(input: &str) -> i32 {
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
            '\n' => line += 1,
            _ => {
                print_error(line, char);
                latest_error_code = 65;
            }
        }
    }

    return latest_error_code;
}

fn print_error(line: i32, token: char) {
    writeln!(
        io::stderr(),
        "[line {}] Error: Unexpected character: {}",
        line,
        token
    )
    .unwrap();
}
