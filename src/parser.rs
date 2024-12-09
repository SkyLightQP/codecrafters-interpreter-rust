use crate::token::Token;

pub fn parse(tokens: Vec<Token>) {
    let mut iter = tokens.iter();

    while let Some(token) = iter.next() {
        match token {
            Token::Reserved(s) => {
                println!("{}", s);
            }
            Token::Number(n) => {
                let symbol = iter.next().unwrap();
                match symbol {
                    Token::Plus | Token::Minus | Token::Star | Token::Slash => {
                        let next = iter.next().unwrap();
                        match next {
                            Token::Number(n2) => {
                                println!("({} {} {})", symbol, n, n2);
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
}
