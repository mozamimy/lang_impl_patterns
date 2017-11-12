use std::fmt;

#[derive(Clone, PartialEq)]
pub enum TokenType {
    NA,
    EOF,
    Name,
    Comma,
    LBrack,
    RBrack,
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let printable = match *self {
            TokenType::NA => "N/A",
            TokenType::EOF => "<EOF>",
            TokenType::Name => "Name",
            TokenType::Comma => "Comma",
            TokenType::LBrack => "LBrack",
            TokenType::RBrack => "RBrack",
        };

        write!(f, "{}", printable)
    }
}

pub struct Token {
    pub token_type: TokenType,
    text: String,
}

impl Token {
    pub fn new(token_type: TokenType, text: String) -> Token {
        Token { token_type: token_type, text: text }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<'{}',{}>", self.text, self.token_type)
    }
}
