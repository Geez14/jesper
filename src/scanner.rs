use crate::token::{self, Literal, Position, Token};

#[derive(Debug, Default)]
pub struct Scanner<ErrorHandler: Fn(Position, String)> {
    src: String,
    filename: String,

    ch: u8,
    offset: usize,
    rd_offset: usize,

    line_offset: usize,
    line_no: usize,

    err: ErrorHandler,
}

impl<E: Fn(Position, String)> Scanner<E> {
    pub fn new(src: String, filename: String, err: E) -> Self {
        let mut l = Self {
            src,
            filename,
            ch: b' ',
            offset: 0,
            rd_offset: 0,

            line_offset: 0,
            line_no: 1,

            err,
        };
        l.next();
        l
    }

    fn next(&mut self) {
        if self.rd_offset < self.src.len() {
            self.offset = self.rd_offset;

            if self.ch == b'\n' {
                self.line_no += 1;
                self.line_offset = self.offset;
            }

            self.ch = self.src.as_bytes()[self.offset];

            self.rd_offset += 1;
        } else {
            self.ch = 0;
            self.offset = self.src.len();
        }
    }

    fn peek(&self) -> u8 {
        *self.src.as_bytes().get(self.rd_offset).unwrap_or(&0)
    }

    fn scan_identifier(&mut self) -> Literal {
        let offset = self.offset;
        let mut lit = String::new();

        self.src
            .as_bytes()
            .iter()
            .skip(offset)
            .filter(move |&&c| is_letter(c))
            .for_each(|&c| lit.push(c as char));

        self.offset += lit.len();
        self.rd_offset = self.offset;

        lit
    }

    fn scan_number(&mut self) -> (Token, Literal) {
        if self.ch == b'0' {
            match self.peek() {
                b'b' => {}
                b'x' | b'X' => {}
                b'0'..=b'7' => {}
                _ => {}
            }
        }

        todo!()
    }

    fn position(&self) -> Position {
        Position {
            filename: self.filename.clone(),
            offset: self.offset,
            line: self.line_no,
            column: self.offset - self.line_offset + 1,
        }
    }

    fn skip_whitespace(&mut self) {
        while self.ch == b' ' || self.ch == b'\t' || self.ch == b'\n' || self.ch == b'\r' {
            self.next();
        }
    }

    fn expect(&mut self, c: u8) -> bool {
        if self.peek() == c {
            self.next();
            true
        } else {
            false
        }
    }

    fn switch(&mut self, def: Token, c: u8, t: Token) -> Token {
        if self.peek() == c {
            self.next();
            t
        } else {
            def
        }
    }

    fn switch2(&mut self, def: Token, c1: u8, t1: Token, c2: u8, t2: Token) -> Token {
        match self.peek() {
            c1 => {
                self.next();
                t1
            }
            c2 => {
                self.next();
                t2
            }
            _ => def,
        }
    }

    fn switch3(
        &mut self,
        def: Token,
        c1: u8,
        t1: Token,
        c2: u8,
        t2: Token,
        c3: u8,
        t3: Token,
    ) -> Token {
        match self.peek() {
            c1 => {
                self.next();
                t1
            }
            c2 => {
                self.next();
                t2
            }
            c3 => {
                self.next();
                t3
            }
            _ => def,
        }
    }

    pub fn scan(&mut self) -> (Token, Position, Literal) {
        self.skip_whitespace();
        let pos = self.position();

        if !self.ch.is_ascii_digit() && is_letter(self.ch) {
            let lit = self.scan_identifier();
            let tok = token::lookup(lit.as_str());
            return (tok, pos, lit);
        }

        if is_digit(self.ch) {
            let (tok, lit) = self.scan_number();
            return (tok, pos, lit);
        }

        let tok = match self.ch {
            b'=' => self.switch(Token::ASSIGN, b'=', Token::EQL),
            b'!' => self.switch(Token::NOT, b'=', Token::NEQ),
            b'+' => self.switch2(Token::PLUS, b'+', Token::INC, b'=', Token::PLUS_ASSIGN),
            b'-' => self.switch3(
                Token::MINUS,
                b'+',
                Token::DEC,
                b'=',
                Token::MINUS_ASSIGN,
                b'>',
                Token::ARROW,
            ),

            b'(' => Token::LPAREN,
            _ => Token::ILLEGAL,
        };

        self.next();

        let lit = self.src[pos.offset..self.offset].to_string();

        (tok, pos, lit)
    }
}

fn is_digit(c: u8) -> bool {
    c >= b'0' && c <= b'9'
}

fn is_letter(c: u8) -> bool {
    c >= b'a' && c <= b'z'
        || c >= b'A' && c <= b'Z'
        || c >= b'0' && c <= b'9'
        || c == b'$'
        || c == b'_'
}
