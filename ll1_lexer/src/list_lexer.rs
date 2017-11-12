use token;

pub struct ListLexer {
    input: String,
    current_chr: Option<char>,
    position: u64,
}

impl ListLexer {
    pub fn new(input: String) -> ListLexer {
        ListLexer {
            input: input.clone(),
            current_chr: input.chars().nth(0),
            position: 0,
        }
    }

    pub fn next_token(&mut self) -> token::Token {
        while self.current_chr.is_some() {
            match self.current_chr.unwrap() {
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
                        panic!("invalid character: {}", self.current_chr.unwrap());
                    }
                },
            }
        }

        token::Token::new(token::TokenType::EOF, String::from("<EOF>"))
    }

    fn name(&mut self) -> token::Token {
        let mut buf = String::new();
        loop {
            buf.push(self.current_chr.unwrap());
            self.consume();

            if !(self.is_letter()) { break }
        }

        token::Token::new(token::TokenType::Name, buf)
    }

    fn white_space(&mut self) {
        loop {
            match self.current_chr.unwrap() {
                ' ' | '\t' | '\n' | '\r' => self.consume(),
                _ => break
            }
        }
    }

    fn consume(&mut self) {
        self.position += 1;

        if self.position >= self.input.len() as u64 {
            self.current_chr = None
        } else {
            self.current_chr = self.input.chars().nth(self.position as usize)
        }
    }

    fn is_letter(&self) -> bool {
        // Return false if current_chr is None
        match self.current_chr.unwrap_or(' ') {
            'a' ... 'z' => true,
            'A' ... 'Z' => true,
            _ => false,
        }
    }
}
