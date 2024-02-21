use crate::token::{self, Literal, Position, Token};

#[derive(Debug, Default)]
pub struct Scanner {
    src: String,
    filename: String,

    ch: u8,
    offset: usize,
    rd_offset: usize,

    line_offset: usize,
    line_no: usize,
}

impl Scanner {
    pub fn new(src: String, filename: String) -> Self {
        let mut l = Self {
            src,
            filename,
            line_no: 1,
            ..Self::default()
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
            .skip(offset + 1)
            .filter(move |&&c| is_letter(c))
            .for_each(|&c| lit.push(c as char));

        self.offset += lit.len();
        self.rd_offset = self.offset;

        lit
    }

    pub fn scan(&mut self) -> (Token, Position, Literal) {
        if !self.ch.is_ascii_digit() && is_letter(self.ch) {
            let offset = self.offset;
            let lit = self.scan_identifier();
            let tok = token::lookup(lit.as_str());

            return (
                tok,
                Position {
                    filename: self.filename.clone(),
                    offset,
                    line: self.line_no,
                    column: self.offset - self.line_offset + 1,
                },
                lit,
            );
        }

        todo!()
    }
}

fn is_letter(c: u8) -> bool {
    c >= b'a' && c <= b'z'
        || c >= b'A' && c <= b'Z'
        || c >= b'0' && c <= b'9'
        || c == b'$'
        || c == b'_'
}
