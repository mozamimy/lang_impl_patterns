use token;

#[derive(Clone)]
pub struct ListLexer {
    input: String,
    lookahead_chr: Option<char>,
    position: u64,
}

impl ListLexer {
    pub fn new(input: String) -> ListLexer {
        ListLexer {
            input: input.clone(),
            lookahead_chr: input.chars().nth(0),
            position: 0,
        }
    }

    pub fn next_token(&mut self) -> token::Token {
        while self.lookahead_chr.is_some() {
            match self.lookahead_chr.unwrap() {
                ' ' | '\t' | '\n' | '\r' => {
                    self.white_space();
                    continue;
                },
                ',' => { self.consume(); return token::Token::new(token::TokenType::Comma, String::from(",")) },
                '[' => { self.consume(); return token::Token::new(token::TokenType::LBrack, String::from("[")) },
                ']' => { self.consume(); return token::Token::new(token::TokenType::RBrack, String::from("]")) },
                _ => {
                    if self.is_letter() {
                        return self.name();
                    } else {
                        panic!("invalid character: {}", self.lookahead_chr.unwrap());
                    }
                },
            }
        }

        token::Token::new(token::TokenType::EOF, String::from("<EOF>"))
    }

    fn name(&mut self) -> token::Token {
        let mut buf = String::new();
        loop {
            buf.push(self.lookahead_chr.unwrap());
            self.consume();

            if !(self.is_letter()) { break }
        }

        token::Token::new(token::TokenType::Name, buf)
    }

    fn white_space(&mut self) {
        loop {
            match self.lookahead_chr.unwrap() {
                ' ' | '\t' | '\n' | '\r' => self.consume(),
                _ => break
            }
        }
    }

    fn consume(&mut self) {
        self.position += 1;

        if self.position >= self.input.len() as u64 {
            self.lookahead_chr = None
        } else {
            self.lookahead_chr = self.input.chars().nth(self.position as usize)
        }
    }

    fn is_letter(&self) -> bool {
        // Return false if lookahead_chr is None
        match self.lookahead_chr.unwrap_or(' ') {
            'a' ... 'z' => true,
            'A' ... 'Z' => true,
            _ => false,
        }
    }
}
