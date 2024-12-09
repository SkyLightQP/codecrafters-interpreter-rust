use crate::token::Token;
use std::io::{self, Write};

pub fn tokenize(input: &str, print_tokens: bool) -> Result<Vec<Token>, i32> {
    let mut line = 1;
    let mut latest_error_code = 0;
    let mut found_single_line_comment = false;
    let mut found_string_literal = false;
    let mut chars = input.chars().peekable();
    let mut string_buffer = String::new();
    let mut tokens: Vec<Token> = vec![];

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
                tokens.push(Token::String(string_buffer.clone()));
                string_buffer = String::new();
                continue;
            }

            string_buffer.push(char);
            continue;
        }

        match char {
            '(' => tokens.push(Token::LeftParen),
            ')' => tokens.push(Token::RightParen),
            '{' => tokens.push(Token::LeftBrace),
            '}' => tokens.push(Token::RightBrace),
            '*' => tokens.push(Token::Star),
            '.' => tokens.push(Token::Dot),
            ',' => tokens.push(Token::Comma),
            '+' => tokens.push(Token::Plus),
            '-' => tokens.push(Token::Minus),
            ';' => tokens.push(Token::Semicolon),
            '=' => {
                if let Some(&'=') = chars.peek() {
                    tokens.push(Token::EqualEqual);
                    chars.next();
                } else {
                    tokens.push(Token::Equal);
                }
            }
            '!' => {
                if let Some(&'=') = chars.peek() {
                    tokens.push(Token::BangEqual);
                    chars.next();
                } else {
                    tokens.push(Token::Bang);
                }
            }
            '>' => {
                if let Some(&'=') = chars.peek() {
                    tokens.push(Token::GreaterEqual);
                    chars.next();
                } else {
                    tokens.push(Token::Greater);
                }
            }
            '<' => {
                if let Some(&'=') = chars.peek() {
                    tokens.push(Token::LessEqual);
                    chars.next();
                } else {
                    tokens.push(Token::Less);
                }
            }
            '/' => {
                if let Some(&'/') = chars.peek() {
                    found_single_line_comment = true;
                } else {
                    tokens.push(Token::Slash);
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
                        number.push(char);
                        chars.next();
                    } else {
                        break;
                    }
                }

                tokens.push(Token::Number(number));
            }
            'a'..='z' | 'A'..='Z' | '_' => {
                let mut identifier = String::new();
                identifier.push(char);

                while let Some(&char) = chars.peek() {
                    if char.is_alphanumeric() || char == '_' {
                        identifier.push(char);
                        chars.next();
                    } else {
                        break;
                    }
                }

                let reserved = [
                    "and", "class", "else", "false", "for", "fun", "if", "nil", "or", "print",
                    "return", "super", "this", "true", "var", "while",
                ];
                if reserved.contains(&identifier.as_str()) {
                    tokens.push(Token::Reserved(identifier));
                } else {
                    tokens.push(Token::Identifier(identifier));
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

    tokens.push(Token::EOF);

    if print_tokens {
        for token in &tokens {
            match token {
                Token::LeftParen => println!("LEFT_PAREN ( null"),
                Token::RightParen => println!("RIGHT_PAREN ) null"),
                Token::LeftBrace => println!("LEFT_BRACE {{ null"),
                Token::RightBrace => println!("RIGHT_BRACE }} null"),
                Token::Star => println!("STAR * null"),
                Token::Dot => println!("DOT . null"),
                Token::Comma => println!("COMMA , null"),
                Token::Plus => println!("PLUS + null"),
                Token::Minus => println!("MINUS - null"),
                Token::Semicolon => println!("SEMICOLON ; null"),
                Token::EqualEqual => println!("EQUAL_EQUAL == null"),
                Token::Equal => println!("EQUAL = null"),
                Token::BangEqual => println!("BANG_EQUAL != null"),
                Token::Bang => println!("BANG ! null"),
                Token::Greater => println!("GREATER > null"),
                Token::GreaterEqual => println!("GREATER_EQUAL >= null"),
                Token::Less => println!("LESS < null"),
                Token::LessEqual => println!("LESS_EQUAL <= null"),
                Token::Slash => println!("SLASH / null"),
                Token::String(s) => println!("STRING \"{}\" {}", s, s.replace("\"", "")),
                Token::Number(s) => println!("NUMBER {} {:?}", s, s.parse::<f64>().unwrap()),
                Token::Identifier(s) => println!("IDENTIFIER {} null", s),
                Token::Reserved(s) => println!("{} {} null", s.to_uppercase(), s),
                Token::EOF => println!("EOF  null"),
            }
        }
    }

    if found_string_literal {
        writeln!(io::stderr(), "[line {}] Error: Unterminated string.", line).unwrap();
        latest_error_code = 65;
    }

    if latest_error_code != 0 {
        return Err(latest_error_code);
    }

    return Ok(tokens);
}
