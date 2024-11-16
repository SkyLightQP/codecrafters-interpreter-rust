use std::io::{self, Write};

pub fn tokenize(input: &str) -> i32 {
    let mut line = 1;
    let mut latest_error_code = 0;
    let mut found_single_line_comment = false;
    let mut found_string_literal = false;
    let mut found_float_literal = false;
    let mut chars = input.chars().peekable();
    let mut string_buffer = String::new();

    while let Some(char) = chars.next() {
        if found_single_line_comment {
            if char == '\n' {
                found_single_line_comment = false;
                line += 1;
            }
            continue;
        }
        if found_string_literal {
            if char == '"' {
                found_string_literal = false;
                println!(
                    "STRING \"{}\" {}",
                    string_buffer,
                    string_buffer.replace("\"", "")
                );
                string_buffer = String::new();
                continue;
            }

            string_buffer.push(char);
            continue;
        }

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
            '/' => {
                if let Some(&'/') = chars.peek() {
                    found_single_line_comment = true;
                } else {
                    println!("SLASH / null");
                }
            }
            '"' => {
                found_string_literal = true;
            }
            '0'..='9' => {
                let mut number = String::new();
                number.push(char);

                while let Some(&char) = chars.peek() {
                    if char.is_digit(10) || char == '.' {
                        if char == '.' {
                            found_float_literal = true;
                        }

                        number.push(char);
                        chars.next();
                    } else {
                        break;
                    }
                }

                if found_float_literal {
                    let s = format!("{:.10}", number);
                    let s = s.trim_end_matches('0').trim_end_matches('.');
                    if !s.contains(".") {
                        println!("NUMBER {} {}.0", number, s);
                        continue;
                    }
                    println!("NUMBER {} {}", number, s);
                    found_float_literal = false;
                } else {
                    println!("NUMBER {} {}.0", number, number);
                }
            }
            '\n' => line += 1,
            ' ' | '\t' => {}
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

    if found_string_literal {
        writeln!(io::stderr(), "[line {}] Error: Unterminated string.", line).unwrap();
        latest_error_code = 65;
    }

    return latest_error_code;
}
