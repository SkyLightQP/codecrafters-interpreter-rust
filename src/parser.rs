use crate::{expression::Expression, token::Token};

pub fn parse(tokens: Vec<Token>) {
    let parsed = Parser::new(tokens).parse();

    println!("{}", parsed);
}

struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }

    fn parse(&mut self) -> Expression {
        self.expression()
    }

    fn expression(&mut self) -> Expression {
        self.binary()
    }

    fn primary(&mut self) -> Expression {
        let token = self.peek();
        match token {
            Token::Number(_) => Expression::Literal(self.advance()),
            Token::String(_) => Expression::Literal(self.advance()),
            Token::Reserved(_) => Expression::Literal(self.advance()),
            Token::LeftParen => {
                self.advance();
                let expr = self.expression();
                if self.peek() == &Token::RightParen {
                    self.advance();
                }
                Expression::Grouping(Box::new(expr))
            }
            _ => panic!("Unexpected token: {:?}", token),
        }
    }

    fn binary(&mut self) -> Expression {
        let mut left = self.unary();
        loop {
            let token = self.peek();
            match token {
                Token::Plus | Token::Minus | Token::Star | Token::Slash => {
                    let operator = self.advance();
                    let right = self.unary();
                    left = Expression::Binary(Box::new(left), operator, Box::new(right));
                }
                _ => break,
            }
        }
        left
    }

    fn unary(&mut self) -> Expression {
        let token = self.peek();
        match token {
            Token::Bang | Token::Minus => {
                let operator = self.advance();
                let right = self.unary();
                return Expression::Unary(operator, Box::new(right));
            }
            _ => {}
        }
        self.primary()
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn advance(&mut self) -> Token {
        if !self.is_end() {
            self.current += 1;
        }
        self.pervious()
    }

    fn pervious(&self) -> Token {
        self.tokens[self.current - 1].clone()
    }

    fn is_end(&self) -> bool {
        self.tokens[self.current] == Token::EOF
    }
}
